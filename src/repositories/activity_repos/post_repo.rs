use serde_json::Value;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::{
    operations::activity_log_ops::post_activity_log::{delete_post_activity, insert_post_activity},
    types::ActivityMetaData,
};

type QueryType = Result<(), sqlx::Error>;
type Trans<'a> = sqlx::Transaction<'a, Sqlite>;

pub async fn handle_store(result: QueryType, tx: Trans<'_>) -> Result<(), Value> {
    match result {
        Ok(()) => {
            tx.commit().await.unwrap();
            Ok(())
        }
        Err(err) => {
            println!("{:?}", err);
            tx.rollback().await.unwrap();
            Err(serde_json::json!(err
                .into_database_error()
                .unwrap()
                .code()
                .unwrap()))
        }
    }
}

pub async fn store_post(
    activity_data: ActivityMetaData,
    database_connection: Pool<Sqlite>,
) -> Result<(), Value> {
    let mut tx = database_connection.begin().await.unwrap();

    let activity_id: Uuid = Uuid::new_v4();
    let activity_id_ref: &[u8] = activity_id.as_ref();

    let query_result = insert_post_activity(activity_data.clone(), activity_id_ref, &mut *tx).await;
    handle_store(query_result, tx).await
}

pub async fn store_delete_post(
    activity_data: ActivityMetaData,
    activity_id_ref: &[u8],
    database_connection: Pool<Sqlite>,
) -> Result<(), Value> {
    let mut tx = database_connection.begin().await.unwrap();
    let query_result = delete_post_activity(activity_data, activity_id_ref, &mut tx).await;
    handle_store(query_result, tx).await
}
