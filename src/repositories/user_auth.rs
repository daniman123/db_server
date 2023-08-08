use actix_web::{web, HttpResponse};

use crate::{
    types::{AppState, UserLoginData},
    utils::tools::generate_token, operations::append_refresh_tokens_table::insert_refresh_tokens,
};
use chrono::Duration;
use serde_json::Value;

#[derive(Debug)]
struct UserExists {
    user_id: Option<i64>,
}

pub async fn handle_login(
    body: web::Json<UserLoginData>,
    state: web::Data<AppState>,
    secret: web::Data<String>,
) -> HttpResponse {

    let database_connection = &state.db.clone();

    let mut tx = database_connection.begin().await.unwrap();


    let db_res = sqlx::query_as!(
        UserExists,
        "SELECT user_id FROM users WHERE username = ?",
        body.username
    )
    .fetch_optional(&mut *tx)
    .await;

    match db_res.unwrap() {
        Some(db_res) => {
            let deez = sqlx::query!(
                "SELECT * FROM user_credentials WHERE user_id = ?",
                db_res.user_id
            )
            .fetch_one(&mut *tx)
            .await;
            let check_pass = deez.unwrap().passphrase;
            let compared_check =
                argon2::verify_encoded(&check_pass, &body.password.as_bytes()).unwrap_or(false);

            if compared_check {
                let id: i32 = db_res.user_id.unwrap().try_into().unwrap();


                let refresh_duration = Duration::days(1);
                let refresh_token = generate_token(id, secret.to_string(), refresh_duration).await;

                insert_refresh_tokens(refresh_token.clone(), id, &mut tx).await.unwrap();

                tx.commit().await.unwrap();


                let access_duration = Duration::minutes(15);
                let access_token =
                    generate_token(id, secret.into_inner().to_string(), access_duration).await;

                let cookie_header = format!("jwt={}; HttpOnly", access_token);
                HttpResponse::Ok()
                .header("Set-Cookie", cookie_header)
                .json(serde_json::json!({"username": body.username, "user_id":id, "access_token":access_token }))
            } else {
                HttpResponse::BadRequest().body("WORNG PASOWRD")
            }
        }
        None => HttpResponse::BadRequest().body("NOT WORK"),
    }
}

pub async fn handle_logout(body: web::Json<Value>, state: web::Data<AppState>) -> HttpResponse {
    
    let user_id = body.0;

    let delete_refresh_token = sqlx::query!("DELETE FROM refreshtokens WHERE user_id = ?", user_id)
        .execute(&state.db.clone())
        .await;

    let cookie_header = format!("jwt=; Max-Age=0; HttpOnly");

    match delete_refresh_token {
        Ok(_) => HttpResponse::Ok()
            .header("Set-Cookie", cookie_header)
            .finish(),
        Err(_) => HttpResponse::Ok()
            .header("Set-Cookie", cookie_header)
            .finish(),
    }
}
