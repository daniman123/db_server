use sqlx::SqliteConnection;
use uuid::Uuid;

use crate::types::ActivityMetaData;

pub async fn insert_into_activity_log(
    activity_id: Uuid,
    activity_data: ActivityMetaData,
    database_connection: &mut SqliteConnection,
) {
    let ActivityMetaData {
        user_id,
        activity_type,
        subject_user_id,
        content,
    } = activity_data;

    let activity_id_ref = activity_id.as_ref();

    sqlx::query!(
        "INSERT INTO activity_log (
            log_id,
            user_id,
            type_name,
            subject_user_id,
            content
        ) 
            VALUES (?,?,?,?,?)",
        activity_id_ref,
        user_id,
        activity_type,
        subject_user_id,
        content,
    )
    .execute(database_connection)
    .await
    .unwrap();
}
