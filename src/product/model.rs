use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub image_url: String,
}
