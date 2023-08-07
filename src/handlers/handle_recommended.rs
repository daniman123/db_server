use actix_web::{ web::{ self }, HttpResponse };
use crate::{ types::{ AppState, JsonResponse }, operations::get_channels::get_recommended };


pub async fn recommended_channels(
    state: web::Data<AppState>
) -> HttpResponse {
    let result = get_recommended(state.db.clone()).await;
    let usernames: Vec<String> = result
        .iter()
        .map(|result| result.username.clone())
        .collect();

    HttpResponse::Ok().json(JsonResponse::new(usernames))
}
