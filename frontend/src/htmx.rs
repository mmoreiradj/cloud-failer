use askama::Template;
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use proto::product_catalog::{ListProductsRequest, Product};

use crate::state::AppState;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error: {}", e),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "shop.html")]
pub struct ShopTemplate {
    pub products: Vec<Product>,
}

#[debug_handler]
pub async fn shop(State(state): State<AppState>) -> impl IntoResponse {
    let response = state
        .product_catalog_client
        .lock()
        .await
        .list_products(ListProductsRequest {})
        .await
        .expect("failed to list products");

    let template = ShopTemplate {
        products: response.into_inner().results,
    };

    HtmlTemplate(template)
}
