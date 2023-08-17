use crate::operations::insert_ops::auth_insert_ops::{
    insert_refresh_tokens, insert_users_credentials_new_user, insert_users_details_new_user,
};
use crate::operations::{
    get_ops::get_by_user_id, insert_ops::auth_insert_ops::insert_users_table_new_user,
};
use crate::types::NewUser;
use crate::utils::tools::generate_token;
use chrono::Duration;
use sqlx::{Pool, Sqlite};

pub async fn store_new_user(
    secret: String,
    prepared_new_user_data: &NewUser,
    database_connection: Pool<Sqlite>,
) -> Result<(String, String), String> {
    let mut tx = match database_connection.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            return Err(format!("Failed to begin transaction: {}", err));
        }
    };

    let mut errors = vec![];

    if let Err(err) = insert_users_table_new_user(&prepared_new_user_data, &mut tx).await {
        errors.push(err);
    }
    if let Err(_) = insert_users_details_new_user(&prepared_new_user_data, &mut tx).await {
        errors.push("Transaction likely terminated".to_string());
    }
    if let Err(err) = insert_users_credentials_new_user(&prepared_new_user_data, &mut tx).await {
        errors.push(err);
    }

    let id = get_by_user_id(&prepared_new_user_data.username, &mut *tx)
        .await
        .unwrap();

    let refresh_duration = Duration::days(1);
    let refresh_token = generate_token(id, secret.clone(), refresh_duration).await;

    if let Err(err) = insert_refresh_tokens(refresh_token.clone(), id, &mut tx).await {
        errors.push(err);
    }

    if !errors.is_empty() {
        tx.rollback().await.unwrap();
        return Err(errors.join("\n"));
    } else {
        tx.commit().await.unwrap();
        let access_duration = Duration::minutes(15);
        let access_token = generate_token(id, secret, access_duration).await;
        return Ok((access_token, refresh_token));
    }
}
