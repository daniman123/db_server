use actix_web::web::{HttpResponse, self};
use serde_json::json;
use crate::{operations::get_ops::get_data, types::AppState};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Resp{
    pub username: String,
}

pub async fn channel_data_handler(
    body: web::Json<Resp>,
    state: web::Data<AppState>
) -> HttpResponse {

    let username = &body.username;

    let database_connection = state.db.clone();

    let mut tx = database_connection.begin().await.unwrap();

    let db_return = get_data(username.to_string(), &mut tx).await.unwrap();
    tx.commit().await.unwrap();

    HttpResponse::Ok().json(json!(db_return))

}