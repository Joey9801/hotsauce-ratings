use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use crate::auth::TokenValidationError;

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
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Forbidden => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        log::error!("{self:?}");
        (self.status_code(), self.to_string()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
