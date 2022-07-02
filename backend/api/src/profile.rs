use axum::{Router, Extension, response::IntoResponse, routing::get, Json};
use entity::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{error::{Result, Error}, auth::AuthenticatedUser};

async fn basic_profile(
    auth: AuthenticatedUser,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse> {
    let user = User::find_by_id(auth.user_id)
        .one(conn)
        .await?
        .ok_or(Error::Unauthorized)?;
        
    Ok(Json(user))
}


pub fn router() -> Router {
    Router::new()
        .route("/basic_profile", get(basic_profile))
}