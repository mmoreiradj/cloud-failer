use std::{path::PathBuf, sync::Arc};

use axum::{routing::get, Router};
use clap::Parser;
use config::FrontendConfig;

use proto::product_catalog::products_client::ProductsClient;
use shared::{init_tracing, trace_layer};
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::services::ServeDir;
use tracing::info;

use crate::state::AppState;
mod config;
mod htmx;
mod state;

#[tokio::main]
async fn main() {
    let args = FrontendConfig::parse();

    init_tracing(args.log_level);

    info!("Starting server on {}", args.bind_address);

    let products_svc = ProductsClient::connect(args.product_catalog_uri.clone())
        .await
        .expect("failed to connect to product catalog");

    let state = AppState {
        frontend_config: args.clone(),
        product_catalog_client: Arc::new(Mutex::new(products_svc)),
    };

    let static_dir = PathBuf::from(args.static_dir.clone());

    let app = Router::new()
        .layer(trace_layer())
        .route("/", get(htmx::shop))
        .nest_service("/static", ServeDir::new(static_dir))
        .with_state(state);

    let listener = TcpListener::bind(args.bind_address)
        .await
        .expect("failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}
