use clap::Parser;
use proto::product_catalog::products_server::ProductsServer;
use shared::init_tracing;
use tonic::transport::Server;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::info;

use crate::{config::ProductCatalogConfig, service::ProductsService};

mod config;
mod service;

#[tokio::main]
async fn main() {
    let args = ProductCatalogConfig::parse();

    init_tracing(args.log_level);

    let products_service = ProductsService::new();

    info!("Starting server on {}", args.bind_address);

    let svc = ProductsServer::new(products_service);

    let grpc_trace_layer = TraceLayer::new_for_grpc()
        .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO))
        .on_failure(DefaultOnFailure::new().level(tracing::Level::ERROR));

    let layer = ServiceBuilder::new().layer(grpc_trace_layer).into_inner();

    Server::builder()
        .layer(layer)
        .add_service(svc)
        .serve(args.bind_address)
        .await
        .expect("failed to start server");
}
