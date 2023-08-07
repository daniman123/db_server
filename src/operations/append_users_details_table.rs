use sqlx::SqliteConnection;
use crate::types::NewUser;

pub async fn insert_users_details_new_user(prepared_new_user_data: &NewUser, 
    database_connection: &mut SqliteConnection)
-> Result<(), String> {
    let users_details_table_insert = sqlx
        ::query!(
            "INSERT INTO user_details (phonetic_username) VALUES (?)",
            prepared_new_user_data.phonetic_username
        )
        .execute(database_connection).await;

        match users_details_table_insert {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
}

