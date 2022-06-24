use axum::{Extension, response::IntoResponse, Json, Router, routing::get, extract::Query};
use sea_orm::prelude::*;

use entity::prelude::*;
use serde::Deserialize;

use crate::error::{Error, Result};

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
        find = find.filter(entity::sauce::Column::Manufacturer.eq(manufacturer_id));
    }

    if let Some(sauce_id) = query.sauce_id {
        find = find.filter(entity::sauce::Column::Id.eq(sauce_id));
    }
    
    find
        .all(conn)
        .await
        .map(|x| Json(x))
        .map_err(Error::from)
}

pub fn router() -> Router {
    Router::new()
        .route("/sauce", get(sauce_list))
}