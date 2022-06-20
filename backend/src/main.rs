use std::{net::SocketAddr, time::Duration};

use anyhow::anyhow;
use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, response::IntoResponse, routing::get,
    BoxError, Extension, Json, Router, extract::Query,
};
use error::Error;
use serde::Deserialize;
use sea_orm::{prelude::*, ConnectOptions, Database, Set};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

mod error;
mod model;

use crate::error::Result;
use model::prelude::*;

async fn manufacturer_list(
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse> {
    Manufacturer::find()
        .all(conn)
        .await
        .map(|x| Json(x))
        .map_err(Error::from)
}

#[derive(Deserialize, Debug)]
struct NewManufacturer {
    pub manufacturer_name: String
}

async fn manufacturer_insert(
    Extension(ref conn): Extension<DatabaseConnection>,
    Json(new): Json<NewManufacturer>,
) -> Result<impl IntoResponse> { 
    model::manufacturer::ActiveModel {
        manufacturer_name: Set(new.manufacturer_name.clone()),
        ..Default::default()
    }
    .save(conn)
    .await?;
    
    let inserted = Manufacturer::find()
        .filter(model::manufacturer::Column::ManufacturerName.eq(new.manufacturer_name))
        .one(conn)
        .await?
        .ok_or(anyhow!("Can't find manufacturer that was just inserted"))?;

    Ok(Json(inserted))
}


fn manufacturer_router() -> Router {
    Router::new().route(
        "/api/manufacturer",
        get(manufacturer_list).put(manufacturer_insert),
    )
}


#[derive(Debug, Deserialize)]
struct SaucesListQuery {
    pub manufacturer_id: Option<i32>,
    pub sauce_id: Option<i32>,
}

async fn sauce_list(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(query): Query<SaucesListQuery>,
) -> Result<impl IntoResponse> {
    let mut find = Sauce::find();
    
    if let Some(manufacturer_id) = query.manufacturer_id {
        find = find.filter(model::sauce::Column::ManufacturerId.eq(manufacturer_id));
    }

    if let Some(sauce_id) = query.sauce_id {
        find = find.filter(model::sauce::Column::SauceId.eq(sauce_id));
    }
    
    find
        .all(conn)
        .await
        .map(|x| Json(x))
        .map_err(Error::from)
}

fn sauce_router() -> Router {
    Router::new()
        .route("/api/sauce", get(sauce_list))
}

async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "hotsauce-api=info");
    }
    env_logger::init();

    let mut opt = ConnectOptions::new("sqlite://hotsauce.db".to_string());
    opt.max_connections(100)
        .sqlx_logging(true);
    let db = Database::connect(opt).await?;
    
    let app = Router::new()
        .merge(manufacturer_router())
        .merge(sauce_router())
        .layer(Extension(db))
        .layer(CorsLayer::permissive())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(10)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
