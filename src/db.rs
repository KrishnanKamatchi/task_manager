use dotenv::dotenv;
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::env;

pub async fn connect_db() -> Pool<Sqlite> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to DB")
}
