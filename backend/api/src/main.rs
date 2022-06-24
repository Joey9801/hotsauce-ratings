use std::{net::SocketAddr, time::Duration};

use axum::{
    error_handling::HandleErrorLayer, http::StatusCode,
    BoxError, Extension, Router
};
use sea_orm::{ConnectOptions, Database};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

mod error;
mod manufacturer;
mod sauce;


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


fn api_router() -> Router {
    Router::new()
        .merge(manufacturer::router())
        .merge(sauce::router())
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
        .nest("/api/v1", api_router())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(10))
                .layer(CorsLayer::permissive())
                .layer(Extension(db)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
