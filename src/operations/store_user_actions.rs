use sqlx::{sqlite::SqliteQueryResult, Error, SqliteConnection};
use crate::types::NewUser;



#[derive(Debug)]
pub struct ActivityLog {
    pub id: i64,
    pub user_id: i64,
    pub action: String,
}


pub async fn store_actions
(prepared_new_user_data: &ActivityLog, 
    database_connection: &mut SqliteConnection)
-> Result<SqliteQueryResult, Error> {
    let store_action = sqlx
        ::query!(
            "INSERT INTO activitylog (user_id, action) VALUES (?, ?)",
            prepared_new_user_data.user_id,
            prepared_new_user_data.action,
        )
        .execute(database_connection).await;

    store_action
}

