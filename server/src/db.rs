use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub async fn establish_connection() -> Result<PgPool, Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL")?;

    Ok(PgPoolOptions::new().max_connections(5).connect(&db_url).await?)
}
