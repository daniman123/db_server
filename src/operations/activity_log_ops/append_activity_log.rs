use crate::types::ActivityMetaData;
use sqlx::SqliteConnection;

pub async fn insert_into_activity_log(
    activity_data: ActivityMetaData,
    activity_id_ref: &[u8],
    database_connection: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    let ActivityMetaData {
        user_id,
        activity_type,
        subject_user_id,
        content,
    } = activity_data;

    let result = sqlx::query!(
        "INSERT INTO activity_log 
        (log_id, user_id, type_name, subject_user_id, content)
         VALUES (?,?,?,?,?)",
        activity_id_ref,
        user_id,
        activity_type,
        subject_user_id,
        content,
    )
    .execute(database_connection)
    .await;
    result.map(|_| ())
}

pub async fn delete_from_activity_log(
    activity_data: ActivityMetaData,
    database_connection: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    let delete = sqlx::query!(
        "DELETE FROM activity_log WHERE user_id = ? AND subject_user_id = ? AND type_name = 'FOLLOW'",
        activity_data.user_id,
        activity_data.subject_user_id
    )
    .execute(database_connection)
    .await;

    delete.map(|_| ())
}

pub async fn get_follow_activity_log(
    activity_data: ActivityMetaData,
    database_connection: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    let is_following = sqlx::
        query!("SELECT user_id FROM activity_log WHERE user_id = ? AND subject_user_id = ? AND type_name = 'FOLLOW'", 
            activity_data.user_id, activity_data.subject_user_id)
            .fetch_one(&mut *database_connection)
            .await;

    match is_following {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

// let news = err.as_database_error().unwrap().code().unwrap();
//             if news == "2067" {
//                 println!("{:?}", news);
//                 sqlx::
//                 query!("DELETE FROM activity_log WHERE user_id = ? AND subject_user_id = ?",
//                     user_id, subject_user_id)
//                     .execute(&mut *database_connection)
//                     .await.
//                     unwrap();

//             }
