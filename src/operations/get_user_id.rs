use sqlx::{ SqliteConnection, Error };

pub async fn get_by_user_id(
    username: &String,
    database_connection: &mut SqliteConnection
) -> Result<i32, Error> {

    let user = sqlx
        ::query!("SELECT user_id FROM users WHERE username = ?", username)
        .fetch_optional(database_connection).await?;

    match user {
        Some(user) => Ok(user.user_id.unwrap().try_into().unwrap()),
        None => Err(Error::RowNotFound),
    }
}
