use actix_web::{web, HttpResponse};

use crate::{
    repositories::store_activity::store_activities,
    types::{ActivityMetaData, AppState},
};

pub async fn activity_logger(
    body: web::Json<ActivityMetaData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let activity_data = body.into_inner();
    let database_connection = state.db.clone();

    store_activities(activity_data, database_connection).await;

    HttpResponse::Ok().body("Activity Stored!")
}
