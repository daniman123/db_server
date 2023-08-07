use sqlx::SqliteConnection;
use crate::types::NewUser;

pub async fn insert_users_credentials_new_user(prepared_new_user_data: &NewUser, 
    database_connection: &mut SqliteConnection)
-> Result<(), String> {
    
    let users_credentials_table_insert = sqlx
        ::query!(
            "INSERT INTO user_credentials (email, passphrase) VALUES (?, ?)",
            prepared_new_user_data.email,
            prepared_new_user_data.passphrase
        )
        .execute(database_connection).await;

    match users_credentials_table_insert {
        Ok(_) => Ok(()),
        Err(_) => Err("Email already tied to existing user".to_string()),
    }
}

