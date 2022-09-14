use std::{net::SocketAddr, time::Duration};

use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    http::{Request, StatusCode},
    BoxError, Extension, Router,
};
use axum_extra::extract::cookie::Key as PrivateCookieKey;
use config::Config;
use http::header::HeaderName;
use sea_orm::{ConnectOptions, Database};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer},
    trace::TraceLayer,
    ServiceBuilderExt,
};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod auth;
mod error;
mod manufacturer;
mod profile;
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
        .merge(auth::router())
        .merge(manufacturer::router())
        .merge(profile::router())
        .merge(sauce::router())
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    database_uri: String,
    private_cookie_key: Option<String>,
}

fn private_cookie_key(config: &AppConfig) -> anyhow::Result<PrivateCookieKey> {
    match config.private_cookie_key.as_ref() {
        Some(b64_key) => {
            let mut decoded = base64::decode(b64_key)?;
            if decoded.len() < 64 {
                log::warn!(
                    "Using private cookie key with only {} bytes of entropy",
                    decoded.len()
                );

                // Just repeat the bytes we have to make the key long enough
                decoded = decoded.iter().cloned().cycle().take(64).collect();
            }
            Ok(PrivateCookieKey::from(&decoded))
        }
        None => {
            log::warn!("Generating new private cookie key - existing cookies will be invalid");
            Ok(PrivateCookieKey::generate())
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let config: AppConfig = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?
        .try_deserialize()?;

    log::info!("Connecting to database \"{}\"", config.database_uri);

    let mut opt = ConnectOptions::new(config.database_uri.clone());
    opt.sqlx_logging(true);
    let db = Database::connect(opt).await?;

    let cookie_key = private_cookie_key(&config)?;

    let x_request_id = HeaderName::from_static("x-request-id");

    let app = Router::new().nest("/api/v1", api_router()).layer(
        ServiceBuilder::new()
            .set_x_request_id(MakeRequestUuid)
            .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .timeout(Duration::from_secs(10))
            .layer(
                TraceLayer::new_for_http().make_span_with(move |req: &Request<Body>| {
                    tracing::debug_span!(
                        "request",
                        uri = %req.uri(),
                        method = %req.method(),
                        request_id = ?req.headers()[&x_request_id]
                    )
                }),
            )
            .layer(Extension(db))
            .layer(Extension(cookie_key)),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    log::info!("Starting up api server on {:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
