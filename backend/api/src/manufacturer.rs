use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
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
    pub name: String,
}

async fn manufacturer_insert(
    Extension(ref conn): Extension<DatabaseConnection>,
    Json(new): Json<NewManufacturer>,
) -> Result<impl IntoResponse> {
    if let Some(existing) = Manufacturer::find()
        .filter(entity::manufacturer::Column::Name.eq(new.name.clone()))
        .one(conn)
        .await?
    {
        return Ok(Json(existing));
    }

    let inserted = entity::manufacturer::ActiveModel {
        name: Set(new.name),
        ..Default::default()
    }
    .insert(conn)
    .await?;

    Ok(Json(inserted))
}

pub fn router() -> Router {
    Router::new().route(
        "/manufacturer",
        get(manufacturer_list).put(manufacturer_insert),
    )
}
