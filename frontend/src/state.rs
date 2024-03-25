use std::sync::Arc;

use proto::product_catalog::products_client::ProductsClient;
use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::config::FrontendConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub frontend_config: FrontendConfig,
    pub product_catalog_client: Arc<Mutex<ProductsClient<Channel>>>,
}
