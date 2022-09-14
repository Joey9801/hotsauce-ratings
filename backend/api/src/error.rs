use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

use crate::auth::{TokenValidationError, UsernameValidationError};

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("A database error occurred")]
    Sqlx(#[from] sqlx::Error),

    #[error("A database error occurred")]
    SeaORM(#[from] sea_orm::DbErr),

    #[error("An internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("An error occured while validating a token")]
    TokenValidationError(#[from] TokenValidationError),

    #[error("There was an issue with a potential new username")]
    UsernameValidationError(#[from] UsernameValidationError),

    #[error("No account exists for the given credentials")]
    NoSuchAccount,

    #[error("The given nonce has been seen before")]
    ReusedNonce,

    #[error("Login required to perform this action")]
    Unauthorized,

    #[error("The given credentials were insufficient to perform this action")]
    Forbidden,
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::TokenValidationError(e) => {
                use TokenValidationError::*;
                match e {
                    JsonWebToken(_) => StatusCode::UNAUTHORIZED,
                    MissingKid | UnknownKey => StatusCode::BAD_REQUEST,
                    Reqwest(_) | UnsupportedKeyAlgorithm(_) => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }
            Error::Unauthorized | Error::NoSuchAccount => StatusCode::UNAUTHORIZED,
            Error::UsernameValidationError(_) => StatusCode::BAD_REQUEST,
            Error::Forbidden => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn detailed_message(&self) -> Option<String> {
        match self {
            Error::UsernameValidationError(e) => Some(e.to_string()),
            _ => None,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        log::error!("{self:?}");
        let msg = json!({
            "error": self.to_string(),
            "details": self.detailed_message(),
        });

        (self.status_code(), Json(msg)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
