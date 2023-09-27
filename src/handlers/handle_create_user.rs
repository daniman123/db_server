use actix_web::{web, HttpResponse};

use crate::repositories::store_new_user::store_new_user;
use crate::types::{AccessTokenRes, AppState, NewUser, NewUserData};

pub async fn create_new_user(
    body: web::Json<NewUserData>,
    state: web::Data<AppState>,
    secret: web::Data<String>,
) -> HttpResponse {
    let verify_credentials = NewUser::new(body);

    if let Ok(prepared_new_user_data) = verify_credentials {
        let store_user = store_new_user(
            secret.into_inner().to_string(),
            &prepared_new_user_data,
            state.db.clone(),
        )
        .await;
        if let Err(err) = store_user {
            return HttpResponse::InternalServerError().body(err);
        }

        let (access_token, refresh_token, id) = store_user.unwrap();

        let cookie_header = format!("jwt={}; HttpOnly", refresh_token);
        HttpResponse::Ok()
            .header("Set-Cookie", cookie_header)
            .json(AccessTokenRes {
                access_token,
                user_id: id,
            })
    } else {
        HttpResponse::BadRequest().body("Could Not Create Account")
    }
}
