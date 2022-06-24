use axum::{response::IntoResponse, http::StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("A database error occurred")]
    Sqlx(#[from] sqlx::Error),

    #[error("A database error occurred")]
    SeaORM(#[from] sea_orm::DbErr),

    #[error("An internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::SeaORM(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match &self {
            Error::Sqlx(e) => log::error!("SQLx error: {:?}", e),
            Error::SeaORM(e) => log::error!("SeaORM error: {:?}", e),
            Error::Anyhow(e) => log::error!("Generic server error: {:?}", e),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;