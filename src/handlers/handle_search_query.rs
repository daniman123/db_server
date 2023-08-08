use actix_web::{ web, HttpResponse };

use crate::types::{ AppState, SearchQuery, JsonResponse };
use crate::operations::get_query_results::get_query;
use crate::utils::tools::metaphone_encoding;

pub async fn search_query(
    body: web::Json<SearchQuery>,
    state: web::Data<AppState>
) -> HttpResponse {
    let user_query = &body.query;
    let metaphoned_q = metaphone_encoding(&user_query);
    let formatted_q = format!("%{q}%", q = metaphoned_q);
    let result = get_query(formatted_q, state.db.clone()).await;

    let usernames: Vec<String> = result
        .iter()
        .map(|result| result.username.clone())
        .collect();

    HttpResponse::Ok().json(JsonResponse::new(usernames))
}
