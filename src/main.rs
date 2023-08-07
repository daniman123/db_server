mod migrations;
mod utils;
mod types;
mod scopes;
mod extractors;
mod operations;
mod repositories;
mod handlers;

use actix_cors::Cors;
use scopes::{ users::user_scope, database_queries::database_queries_scope };
use actix_web::{ http, web::{ self, Data }, App, HttpServer };
use types::AppState;
use utils::db::create_sqlite_pool;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_sqlite_pool().await.unwrap();

    // Define the Actix Web app and routes
    let app = move ||
        App::new()
            .wrap(
                Cors::default()
                    // .allow_any_origin()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
                    .supports_credentials()
            )
            .app_data(Data::new(AppState { db: pool.clone() }))
            .app_data(
                web::Data::new(
                    String::from("$2a$12$C883CDpxYjDDgHVRrjzwL.YpiViCFDA9qwOd5IHXOoax3qJCaaoz2")
                )
            )
            .service(user_scope())
            .service(database_queries_scope());

    // Start the Actix Web server
    HttpServer::new(app).bind("localhost:9000")?.run().await?;

    Ok(())
}
