use crate::types::{AppState, ChannelNames};
use actix_web::web;
use sqlx::{self};

#[derive(sqlx::FromRow, Debug)]
struct Username {
    subject_user_id: Option<i64>,
}

pub async fn get_following(user_id: i64, state: web::Data<AppState>) -> Vec<ChannelNames> {
    let database_connection = &state.db.clone();

    let mut tx = database_connection.begin().await.unwrap();

    let user_ids = sqlx::query_as!(
        Username,
        "SELECT subject_user_id FROM activity_log WHERE user_id = ? AND type_name = 'FOLLOW'",
        user_id
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap();

    let user_ids: Vec<Option<i64>> = user_ids
        .into_iter()
        .map(|user| user.subject_user_id)
        .collect();

    let ids_placeholder: String = user_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT username FROM users WHERE user_id IN ({})",
        ids_placeholder
    );

    let mut query = sqlx::query_as(&sql);

    for user_id in &user_ids {
        query = query.bind(user_id);
    }

    let values: Vec<ChannelNames> = query.fetch_all(&mut *tx).await.unwrap();

    values
}
