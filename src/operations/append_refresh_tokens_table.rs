use sqlx::SqliteConnection;


pub async fn insert_refresh_tokens(
    refresh_token: String,
    user_id: i32, 
    database_connection: &mut SqliteConnection)
-> Result<(), String> {
    
    let users_credentials_table_insert = sqlx
        ::query!(
            "INSERT INTO refreshtokens (refresh_token, user_id) VALUES (?, ?)",
            refresh_token,
            user_id
        )
        .execute(database_connection).await;

    match users_credentials_table_insert {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

