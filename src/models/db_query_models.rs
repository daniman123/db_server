use actix_web::web;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Pool, Sqlite};

use crate::{
    types::{ActivityMetaData, AppState},
    types_::db_query_types::ChannelDataVecType,
};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ChannelData {
    pub user_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonChannelDataResponse {
    pub result: ChannelDataVecType,
}

impl JsonChannelDataResponse {
    pub fn _new(database_return: ChannelDataVecType) -> Self {
        JsonChannelDataResponse {
            result: database_return,
        }
    }
}

pub trait ActivityContext {
    fn build_from_req(
        body: web::Json<ActivityMetaData>,
        state: web::Data<AppState>,
    ) -> ActivityContextBuilder;
}

pub struct ActivityContextBuilder {
    pub activity_data: ActivityMetaData,
    pub database_connection: Pool<Sqlite>,
}

impl ActivityContext for ActivityContextBuilder {
    fn build_from_req(
        body: web::Json<ActivityMetaData>,
        state: web::Data<AppState>,
    ) -> ActivityContextBuilder {
        ActivityContextBuilder {
            activity_data: body.into_inner(),
            database_connection: state.db.clone(),
        }
    }
}

use std::future::Future;

pub async fn operation_executor<F, Fut>(
    body: web::Json<ActivityMetaData>,
    state: web::Data<AppState>,
    storage_operation: F,
) -> HttpResponse
where
    F: Fn(ActivityMetaData, Pool<Sqlite>) -> Fut,
    Fut: Future<Output = Result<(), Value>>,
{
    let activity_data = body.into_inner();
    println!("{:?}", activity_data);
    let database_connection = state.db.clone();

    let store = storage_operation(activity_data, database_connection).await;

    match store {
        Ok(_) => HttpResponse::Ok().body("Activity Stored!"),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}
