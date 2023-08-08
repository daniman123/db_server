use sqlx::{ Sqlite, Pool};

use crate::types::ChannelNames;


pub async fn get_recommended(
    database_connection: Pool<Sqlite>
)-> Vec<ChannelNames>{
    
    let mut tx: sqlx::Transaction<'_, Sqlite> = database_connection.begin().await.unwrap();

    let query_return = sqlx
        ::query_as!(ChannelNames, "SELECT username FROM users ORDER BY RANDOM() LIMIT 10")
        .fetch_all(&mut *tx).await
        .unwrap();

    query_return
}   