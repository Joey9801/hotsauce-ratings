use axum::{
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    PrivateCookieJar,
};
use chrono::{DateTime, Duration, Utc};
use entity::prelude::*;
use jsonwebtoken::{
    decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    DecodingKey, Validation,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::error::{Error, Result};

const GOOGLE_CLIENT_ID: &'static str =
    "1029137063431-crnebmaeal8jdm85iurqoin9k6aqvccj.apps.googleusercontent.com";

#[derive(Debug, Error)]
pub enum TokenValidationError {
    #[error("There was an issue while actually validating the token")]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),

    #[error("There was an issue with a secondary request")]
    Reqwest(#[from] reqwest::Error),

    #[error("The token used a key algorithm that we don't support yet")]
    UnsupportedKeyAlgorithm(AlgorithmParameters),

    #[error("The token did not specify a key id")]
    MissingKid,

    #[error("Can't find the tokens key id in any known jwks")]
    UnknownKey,
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
}

async fn validate_token(token: &str) -> std::result::Result<Claims, TokenValidationError> {
    // TODO: save these keys somewher rather than fetch them on every single validation
    let keys: JwkSet = reqwest::get("https://www.googleapis.com/oauth2/v3/certs")
        .await?
        .json()
        .await?;

    let header = decode_header(token)?;

    let kid = header
        .kid
        .as_deref()
        .ok_or(TokenValidationError::MissingKid)?;

    let key = keys.find(kid).ok_or(TokenValidationError::UnknownKey)?;
    let decoding_key = match &key.algorithm {
        AlgorithmParameters::RSA(params) => DecodingKey::from_rsa_components(&params.n, &params.e)?,
        _ => Err(TokenValidationError::UnsupportedKeyAlgorithm(
            key.algorithm.clone(),
        ))?,
    };

    let mut validation = Validation::new(header.alg);
    validation.validate_exp = true;
    validation.validate_nbf = true;
    validation.set_audience(&[GOOGLE_CLIENT_ID]);
    validation.set_issuer(&["https://accounts.google.com"]);

    let decoded = jsonwebtoken::decode::<Claims>(&token, &decoding_key, &validation)?;

    Ok(decoded.claims)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginCookieData {
    pub user_id: i32,
    pub logged_in_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}

impl LoginCookieData {
    pub fn encode_cookie_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn decode_cookie_str(s: &str) -> Option<Self> {
        serde_json::from_str(s).ok()
    }
}

async fn create_user(
    conn: &DatabaseConnection,
    claims: &Claims,
) -> std::result::Result<i32, DbErr> {
    let new_user = entity::user::ActiveModel {
        name: Set(claims.name.clone()),
        given_name: Set(claims.given_name.clone()),
        family_name: Set(claims.family_name.clone()),
        email: Set(claims.email.clone()),
        ..Default::default()
    }
    .insert(conn)
    .await?;

    entity::user_google_login::ActiveModel {
        user: Set(new_user.id),
        google_id: Set(claims.sub.clone()),
    }
    .insert(conn)
    .await?;

    Ok(new_user.id)
}

async fn get_user_id(
    conn: &DatabaseConnection,
    claims: &Claims,
) -> std::result::Result<i32, DbErr> {
    let google_id = &claims.sub;

    match UserGoogleLogin::find_by_id(google_id.clone())
        .one(conn)
        .await?
    {
        Some(ugl) => Ok(ugl.user),
        None => create_user(conn, claims).await,
    }
}

#[derive(Debug, Deserialize)]
struct LoginQueryParams {
    google_access_token: String,
}

async fn login(
    Extension(ref conn): Extension<DatabaseConnection>,
    jar: PrivateCookieJar,
    Json(params): Json<LoginQueryParams>,
) -> Result<impl IntoResponse> {
    let claims = validate_token(&params.google_access_token).await?;

    let now = Utc::now();
    let cookie_data = LoginCookieData {
        user_id: get_user_id(conn, &claims).await?,
        logged_in_at: now,
        valid_until: now + Duration::days(7),
    };

    let login_cookie = Cookie::build("login_cookie", cookie_data.encode_cookie_str())
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/api")
        .finish();

    let jar = jar.add(login_cookie);

    Ok((jar, "Successful login"))
}

async fn debug_login_cookie(jar: PrivateCookieJar) -> Result<impl IntoResponse> {
    let cookie = jar.get("login_cookie").ok_or(Error::Unauthorized)?;
    let cookie_data =
        LoginCookieData::decode_cookie_str(cookie.value()).ok_or(Error::Unauthorized)?;

    Ok(Json(cookie_data))
}

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/debug_login_cookie", get(debug_login_cookie))
}
