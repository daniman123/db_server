use crate::operations::model_ops::BroadCastActivity;
use crate::types::ActivityMetaData;
use sqlx::SqliteConnection;

pub async fn insert_broadcast_activity(
    activity_data: ActivityMetaData,
    activity_id_ref: &[u8],
    database_connection: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    let data = match BroadCastActivity::new(activity_data).await {
        Some(data) => data,
        None => return Err(sqlx::Error::PoolClosed),
    };

    let result = sqlx::query_as!(
        BroadCastActivity,
        "INSERT INTO activity_log 
        (log_id, user_id, type_name, subject_user_id, content)
         VALUES (?,?,?,?,?)",
        activity_id_ref,
        data.user_id,
        data.activity_type,
        data.subject_user_id,
        data.content,
    )
    .execute(database_connection)
    .await;
    result.map(|_| ())
}

pub async fn delete_broadcast_activity(
    activity_data: ActivityMetaData,
    database_connection: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    let delete = sqlx::query!(
        "DELETE FROM activity_log WHERE user_id = ? AND type_name = 'BROADCAST'",
        activity_data.user_id,
    )
    .execute(database_connection)
    .await;

    delete.map(|_| ())
}
