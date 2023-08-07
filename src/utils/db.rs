use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

const DATABASE_URL: &str = "sqlite://database.sqlite";
const MAX_CONNECTIONS: u32 = 10;

pub async fn create_sqlite_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(DATABASE_URL)
        .await?;
    Ok(pool)
}
