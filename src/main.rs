mod entity;
mod result;
mod route;

use axum::error_handling::HandleErrorLayer;
use axum::{http::StatusCode, routing::get, Router};
use entity::events;
use sea_orm::entity::prelude::*;
use sea_orm::{ConnectOptions, Database};
use std::{net::SocketAddr, time::Duration};
use tower::{BoxError, ServiceBuilder};
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "backend=debug,tower_http=debug")
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let db_connection_str =
        std::env::var("DATABASE_URL").expect("export DATABASE_URL");
    // setup connection pool
    let mut opts = ConnectOptions::new(db_connection_str);
    opts.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

    let pool: DatabaseConnection = Database::connect(opts)
        .await
        .expect("faled to connect to the database");

    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|error: BoxError| async move {
            if error.is::<tower::timeout::error::Elapsed>() {
                Ok(StatusCode::REQUEST_TIMEOUT)
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }))
        .timeout(Duration::from_secs(10))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(pool));

    // Compose the routes
    let api = Router::new().route("/events", get(route::events::list));

    // Compose the app
    let app = Router::new().nest("/api/v1", api).layer(middleware_stack);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
