use std::sync::Arc;

use proto::product_catalog::products_client::ProductsClient;
use tokio::sync::Mutex;
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub struct AppState {
    pub product_catalog_client: Arc<Mutex<ProductsClient<Channel>>>,
}
