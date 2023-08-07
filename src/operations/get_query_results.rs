// use sqlx::{ Pool, Sqlite };

// use crate::types::DbResult;

// pub async fn get_query(query: String, database_connection: Pool<Sqlite>) -> Vec<DbResult> {
//     let mut tx: sqlx::Transaction<'_, Sqlite> = database_connection.begin().await.unwrap();

//     let query_return = sqlx
//         ::query_as!(DbResult, "SELECT username FROM users WHERE phonetic_username LIKE ? LIMIT 7", query)
//         .fetch_all(&mut *tx).await
//         .unwrap();

//     query_return
// }
