use sqlx::{ Pool, Sqlite };

use crate::types::{ChannelNames, DbResult};



pub async fn get_query(query: String, database_connection: Pool<Sqlite>) -> Vec<ChannelNames> {
    let mut tx: sqlx::Transaction<'_, Sqlite> = database_connection.begin().await.unwrap();

    let user_ids: Vec<DbResult> = sqlx
    ::query_as!(DbResult, "SELECT user_id FROM user_details WHERE phonetic_username LIKE ? LIMIT 7", query)
    .fetch_all(&mut *tx).await
    .unwrap();

    let user_ids: Vec<i64> = user_ids.into_iter().map(|user| user.user_id).collect();

    let ids_placeholder: String = user_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!("SELECT username FROM users WHERE user_id IN ({})", ids_placeholder);

    let mut query = sqlx::query_as(&sql);

    for user_id in &user_ids {
        query = query.bind(user_id);
    }

    let values: Vec<ChannelNames> = query
        .fetch_all(&mut *tx)
        .await.unwrap();

    values

}
