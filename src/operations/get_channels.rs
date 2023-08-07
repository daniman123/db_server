use sqlx::{ Sqlite, Pool};

use crate::types::DbResult;


pub async fn get_recommended(
    database_connection: Pool<Sqlite>
)-> Vec<DbResult>{
    
    let mut tx: sqlx::Transaction<'_, Sqlite> = database_connection.begin().await.unwrap();

    let query_return = sqlx
        ::query_as!(DbResult, "SELECT username FROM users ORDER BY RANDOM() LIMIT 10")
        .fetch_all(&mut *tx).await
        .unwrap();

    query_return
}   