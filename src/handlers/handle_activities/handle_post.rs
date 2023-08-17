use crate::models::db_query_models::operation_executor;
use crate::repositories::activity_repos::post_repo::{store_delete_post, store_post};
use crate::types::ActivityMetaData;
use crate::types::AppState;
use actix_web::{web, HttpResponse};

pub async fn post_logger(
    body: web::Json<ActivityMetaData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    operation_executor(body, state, |data, conn| store_post(data, conn)).await
}

pub async fn delete_post_logger(
    path: web::Path<String>,
    body: web::Json<ActivityMetaData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let activity_data = body.into_inner();
    let database_connection = state.db.clone();

    let store = store_delete_post(activity_data, path.as_bytes(), database_connection).await;

    match store {
        Ok(_) => HttpResponse::Ok().body("Activity Stored!"),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}
