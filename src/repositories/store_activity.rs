use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::{types::ActivityMetaData, operations::activity_log_ops::append_activity_log::insert_into_activity_log};

pub async fn store_activities(activity_data: ActivityMetaData, database_connection: Pool<Sqlite>) {
    let activity_id: Uuid = Uuid::new_v4();

    let mut tx = database_connection.begin().await.unwrap();

    insert_into_activity_log(activity_id.clone(), activity_data, &mut tx).await;

    tx.commit().await.unwrap();
}
