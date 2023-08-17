mod extractors;
mod handlers;
mod migrations;
mod operations;
mod repositories;
mod scopes;
mod types;
mod types_;
mod utils;
mod models;

// use color_eyre::Result;
use actix_cors::Cors;
use actix_web::{
    http,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use scopes::{database_queries::database_queries_scope, users::user_scope, activities::activites_scope};
use types::AppState;
use utils::db::create_sqlite_pool;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_sqlite_pool().await.unwrap();

    let app = move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
                    .supports_credentials(),
            )
            .app_data(Data::new(AppState { db: pool.clone() }))
            .app_data(web::Data::new(String::from(
                "$2a$12$C883CDpxYjDDgHVRrjzwL.YpiViCFDA9qwOd5IHXOoax3qJCaaoz2",
            )))
            .service(user_scope())
            .service(database_queries_scope())
            .service(activites_scope())
    };

    // Start the Actix Web server
    HttpServer::new(app).bind("localhost:9000")?.run().await?;

    Ok(())
}
