use axum::{Router, Extension, response::IntoResponse, routing::get, Json};
use axum_extra::extract::PrivateCookieJar;
use entity::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{error::{Result, Error}, auth::LoginCookieData};

async fn basic_profile(
    Extension(ref conn): Extension<DatabaseConnection>,
    jar: PrivateCookieJar,
) -> Result<impl IntoResponse> {
    let cookie = jar.get("login_cookie").ok_or(Error::Unauthorized)?;
    let cookie_data =
        LoginCookieData::decode_cookie_str(cookie.value()).ok_or(Error::Unauthorized)?;

    let user = User::find_by_id(cookie_data.user_id)
        .one(conn)
        .await?
        .ok_or(Error::Unauthorized)?;
        
    Ok(Json(user))
}


pub fn router() -> Router {
    Router::new()
        .route("/basic_profile", get(basic_profile))
}