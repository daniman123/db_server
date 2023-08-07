use sqlx::SqliteConnection;
use crate::types::NewUser;

pub async fn insert_users_table_new_user(prepared_new_user_data: &NewUser, 
    database_connection: &mut SqliteConnection)
-> Result<(), String> {
    let users_table_insert = sqlx
        ::query!(
            "INSERT INTO users (username) VALUES (?)",
            prepared_new_user_data.username)
        .execute(database_connection).await;

    match users_table_insert {
        Ok(_) => Ok(()),
        Err(_) => Err("Username already exists".to_string()),
    }
}

