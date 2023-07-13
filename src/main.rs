use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let database_url = std::env::var("DATABASE_URL").expect("Please set the DATABASE_URL env var");

    /* let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .expect("Could not connect to the database"); */
}
