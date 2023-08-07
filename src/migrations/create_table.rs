use sqlx::{ Executor, sqlite::SqliteConnection };

pub async fn _table_migration(mut conn: SqliteConnection) {
    let query =
        "
        CREATE TABLE users (
            user_id INTEGER PRIMARY KEY AUTOINCREMENT,
            username VARCHAR(255) NOT NULL,
            phonetic_username VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL,
            passphrase VARCHAR(255) NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
    
        );
    ";

    let _ = conn.execute(query).await;
}
