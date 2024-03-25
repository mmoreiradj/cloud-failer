use sqlx::PgPool;

use super::model::Product;

pub async fn test_connection(pool: &PgPool) -> Result<(), sqlx::Error> {
    let _ = pool.acquire().await?;
    Ok(())
}

pub async fn get_products(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    sqlx::query_as!(
        Product,
        r#"
        SELECT
            id,
            name,
            price,
            image_url
        FROM
            products
        "#,
    )
    .fetch_all(pool)
    .await
}
