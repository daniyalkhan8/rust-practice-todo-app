use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use dotenvy;

pub async fn establish_connection() -> anyhow::Result<PgPool> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .connect(&db_url)
        .await?;

    Ok(pool)
}
