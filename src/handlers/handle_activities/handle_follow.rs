use crate::models::db_query_models::{operation_executor, ActivityContext, ActivityContextBuilder};
use crate::repositories::activity_repo::{get_is_following, store_follow, store_unfollow};
use crate::types::ActivityMetaData;
use crate::types::AppState;
use actix_web::{web, HttpResponse};

pub async fn follow_logger(
    body: web::Json<ActivityMetaData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    operation_executor(body, state, |data, conn| store_follow(data, conn)).await
}

pub async fn unfollow_logger(
    body: web::Json<ActivityMetaData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    operation_executor(body, state, |data, conn| store_unfollow(data, conn)).await
}

pub async fn is_following_logger(
    body: web::Json<ActivityMetaData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let context: ActivityContextBuilder =
        <ActivityContextBuilder as ActivityContext>::build_from_req(body, state);
    let store = get_is_following(context.activity_data, context.database_connection).await;

    match store {
        Ok(following) => HttpResponse::Ok().body(following.to_string()),
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}
