use actix_web::{web, Scope};

use crate::handlers::{
    handle_activities::activity_logger, handle_create_user::create_new_user,
    handle_recommended::recommended_channels, handle_search_query::search_query,
};

pub fn database_queries_scope() -> Scope {
    web::scope("/database-queries")
        .route("/create-new-user", web::post().to(create_new_user))
        .route("/search-query", web::post().to(search_query))
        .route("/recommended-channels", web::get().to(recommended_channels))
        .route("/activities", web::post().to(activity_logger))
}
