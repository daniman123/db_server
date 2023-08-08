use crate::types::AppState;
use actix_web::web;
use sqlx::{self};

#[derive(sqlx::FromRow)]
struct Username {
    username: Option<String>,
}

pub async fn get_username(user_id: i32, state: web::Data<AppState>) -> String {
    let database_connection = &state.db.clone();

    let mut tx = database_connection.begin().await.unwrap();

    let username_row = sqlx::query_as!(
        Username,
        "SELECT username FROM users WHERE user_id = ?",
        user_id
    )
    .fetch_optional(&mut *tx)
    .await;

    tx.commit().await.unwrap();

    // Extract the username string from the username_row Option
    let username = username_row.map(|row| row.unwrap().username).unwrap().unwrap();

    username
}
