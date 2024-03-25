use axum::routing::{get, Router};
use clap::Parser;
use cloud_failer::{init_tracing, trace_layer};
use product::args::Args;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{sync::Arc, time::Duration};
use thiserror::Error;
use tokio::net::TcpListener;
use tracing::{error, info, Instrument};

use crate::product::{
    handlers::{live, ready},
    state::AppState,
};

mod product;

async fn create_connection_pool(database_url: String) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url.as_str())
        .await
}

#[derive(Error, Debug)]
enum AppError {
    #[error("failed to connect to postgres: {0}")]
    ConnectionError(#[from] sqlx::Error),
    #[error("failed to bind to address: {0}")]
    BindError(#[from] std::io::Error),
    #[error("failed to run migrations: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}

#[tracing::instrument]
#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = Args::parse();

    init_tracing();

    info!(
        "connecting to postgres at {}:{} with user {}",
        config.postgres_host, config.postgres_port, config.postgres_user
    );

    let pool = create_connection_pool(
        format!(
            "postgres://{}:{}@{}:{}/{}",
            config.postgres_user,
            config.postgres_password,
            config.postgres_host,
            config.postgres_port,
            config.postgres_database
        )
        .to_string(),
    )
    .await
    .map_err(|e| {
        error!("failed to connect to postgres: {}", e);
        AppError::ConnectionError(e)
    })?;

    let _ = sqlx::migrate!("db/product/migrations")
        .run(&pool)
        .in_current_span()
        .await
        .map_err(|e| {
            error!("failed to run migrations: {}", e);
            e
        })?;

    let state = Arc::new(AppState {
        pool,
        app_config: config.clone(),
    });

    let app = Router::new()
        .route("/health/live", get(live))
        .route("/health/ready", get(ready))
        .route("/products", get(product::handlers::get_products))
        .with_state(state)
        .layer(trace_layer());

    info!("server starting on {}", config.bind_address);

    let listener = TcpListener::bind(config.bind_address).await.map_err(|e| {
        error!("failed to bind to address: {}", e);
        AppError::BindError(e)
    })?;

    axum::serve(listener, app).await.map_err(|e| {
        error!("server error: {}", e);
        AppError::BindError(e)
    })?;

    Ok(())
}
