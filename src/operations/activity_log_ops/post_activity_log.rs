use crate::operations::model_ops::PostActivity;
use crate::types::ActivityMetaData;
use sqlx::SqliteConnection;

pub async fn insert_post_activity(
    activity_data: ActivityMetaData,
    activity_id_ref: &[u8],
    database_connection: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    let data = PostActivity::new(activity_data).await.unwrap();
    
    let result = sqlx::query_as!(
        PostActivity,
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

pub async fn delete_post_activity(
    activity_data: ActivityMetaData,
    activity_id_ref: &[u8],
    database_connection: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    let delete = sqlx::query!(
        "DELETE FROM activity_log WHERE user_id = ? AND log_id = ? AND type_name = 'POST'",
        activity_data.user_id,
        activity_id_ref
    )
    .execute(database_connection)
    .await;

    delete.map(|_| ())
}
