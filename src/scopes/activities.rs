use actix_web::{web, Scope};

use crate::handlers::handle_activities::{
    handle_follow::{follow_logger, is_following_logger, unfollow_logger},
    handle_post::{delete_post_logger, post_logger},
};

pub fn activites_scope() -> Scope {
    web::scope("/activities")
        .route("/follow", web::post().to(follow_logger))
        .route("/unfollow", web::post().to(unfollow_logger))
        .route("/isfollowing", web::post().to(is_following_logger))
        .route("/post", web::post().to(post_logger))
        .route("/post-delete/{id}", web::post().to(delete_post_logger))
}
