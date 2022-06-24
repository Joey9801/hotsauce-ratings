use anyhow::anyhow;
use axum::{Extension, response::IntoResponse, Json, Router, routing::get};
use sea_orm::{prelude::*, Set};

use entity::prelude::*;
use serde::Deserialize;

use crate::error::{Error, Result};


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
    pub name: String
}

async fn manufacturer_insert(
    Extension(ref conn): Extension<DatabaseConnection>,
    Json(new): Json<NewManufacturer>,
) -> Result<impl IntoResponse> { 
    entity::manufacturer::ActiveModel {
        name: Set(new.name.clone()),
        ..Default::default()
    }
    .save(conn)
    .await?;
    
    let inserted = Manufacturer::find()
        .filter(entity::manufacturer::Column::Name.eq(new.name))
        .one(conn)
        .await?
        .ok_or(anyhow!("Can't find manufacturer that was just inserted"))?;

    Ok(Json(inserted))
}


pub fn router() -> Router {
    Router::new().route(
        "/manufacturer",
        get(manufacturer_list).put(manufacturer_insert),
    )
}