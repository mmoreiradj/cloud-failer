use proto::product_catalog::{
    products_server::Products, ListProductsRequest, ListProductsResponse, Product,
};
use tonic::{Request, Response};

pub struct ProductsService {}

type Result<T> = std::result::Result<Response<T>, tonic::Status>;

#[tonic::async_trait]
impl Products for ProductsService {
    async fn list_products(
        &self,
        _request: Request<ListProductsRequest>,
    ) -> Result<ListProductsResponse> {
        let response = ListProductsResponse {
            results: vec![Product {
                id: "8bda6c3c-f1e1-440a-8992-bf72f6ecbf86".to_string(),
                name: "Pastis de Marseille".to_string(),
                image_url: "https://media.carrefour.fr/medias/14fb82fc10dc31b780e7cc6f8f88e33e/p_1500x1500/03163937010003-h1n1-s01.jpg".to_string(),
                price: 40.0,
            },
            Product {
                id: "3aa5aec5-d2ac-4067-bffc-f611c3f2b8e8".to_string(),
                name: "Absolut Vodka".to_string(),
                image_url: "https://media.carrefour.fr/medias/86703aca12cf3a2aa81a9716c0f6cbe5/p_1500x1500/07312040017683-h1n1-s02.jpg".to_string(),
                price: 20.0,
            }],
        };

        Ok(Response::new(response))
    }
}

impl ProductsService {
    pub fn new() -> Self {
        ProductsService {}
    }
}
