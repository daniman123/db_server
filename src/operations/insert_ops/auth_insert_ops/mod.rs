use crate::types::NewUser;
use sqlx::SqliteConnection;

pub async fn insert_refresh_tokens(
    refresh_token: String,
    user_id: i32,
    database_connection: &mut SqliteConnection,
) -> Result<(), String> {
    let users_credentials_table_insert = sqlx::query!(
        "INSERT OR REPLACE INTO refreshtokens (refresh_token, user_id) VALUES (?, ?)",
        refresh_token,
        user_id
    )
    .execute(database_connection)
    .await;

    match users_credentials_table_insert {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn insert_users_credentials_new_user(
    prepared_new_user_data: &NewUser,
    database_connection: &mut SqliteConnection,
) -> Result<(), String> {
    let users_credentials_table_insert = sqlx::query!(
        "INSERT INTO user_credentials (email, password) VALUES (?, ?)",
        prepared_new_user_data.email,
        prepared_new_user_data.password
    )
    .execute(database_connection)
    .await;

    match users_credentials_table_insert {
        Ok(_) => Ok(()),
        Err(_) => Err("Email already tied to existing user".to_string()),
    }
}

pub async fn insert_users_details_new_user(
    prepared_new_user_data: &NewUser,
    database_connection: &mut SqliteConnection,
) -> Result<(), String> {
    let users_details_table_insert = sqlx::query!(
        "INSERT INTO user_details (phonetic_username) VALUES (?)",
        prepared_new_user_data.phonetic_username
    )
    .execute(database_connection)
    .await;

    match users_details_table_insert {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn insert_users_table_new_user(
    prepared_new_user_data: &NewUser,
    database_connection: &mut SqliteConnection,
) -> Result<(), String> {
    let users_table_insert = sqlx::query!(
        "INSERT INTO users (username) VALUES (?)",
        prepared_new_user_data.username
    )
    .execute(database_connection)
    .await;

    match users_table_insert {
        Ok(_) => Ok(()),
        Err(_) => Err("Username already exists".to_string()),
    }
}
