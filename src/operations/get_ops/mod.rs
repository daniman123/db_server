pub mod get_protected_ops;
pub mod get_query_ops;

use crate::models::db_query_models::ChannelData;
use crate::types::ChannelNames;
use sqlx::{Error, SqliteConnection};
use sqlx::{Pool, Sqlite};

pub async fn get_data(
    username: String,
    database_connection: &mut SqliteConnection,
) -> Result<i64, Error> {
    let user = sqlx::query_as!(
        ChannelData,
        "SELECT user_id FROM users WHERE username = ?",
        username
    )
    .fetch_one(database_connection)
    .await?;

    match user.user_id {
        Some(user) => Ok(user),
        None => Err(Error::RowNotFound),
    }
}

pub async fn get_recommended(database_connection: Pool<Sqlite>) -> Vec<ChannelNames> {
    let mut tx: sqlx::Transaction<'_, Sqlite> = database_connection.begin().await.unwrap();

    let query_return = sqlx::query_as!(
        ChannelNames,
        "SELECT username FROM users ORDER BY RANDOM() LIMIT 10"
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap();

    query_return
}

pub async fn get_by_user_id(
    username: &String,
    database_connection: &mut SqliteConnection,
) -> Result<i32, Error> {
    let user = sqlx::query!("SELECT user_id FROM users WHERE username = ?", username)
        .fetch_optional(database_connection)
        .await?;

    match user {
        Some(user) => Ok(user.user_id.unwrap().try_into().unwrap()),
        None => Err(Error::RowNotFound),
    }
}

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
    let username = username_row
        .map(|row| row.unwrap().username)
        .unwrap()
        .unwrap();

    username
}
