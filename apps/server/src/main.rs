mod config;
mod handlers;
mod model;
mod repo;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, http::header, web};
use config::Config;
use dotenv::dotenv;
use model::AppState;
use neo4rs::{Graph, query};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = Config::init().database_url.to_string();
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let graph = Graph::new("neo4j://localhost:7687", "neo4j", "12345678")
        .await
        .unwrap();

    let db = AppState::init(pool, graph);
    let app_data = web::Data::new(db);
    let public_dir = std::env::current_dir().unwrap().join("public");

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:5173")
            .allow_any_method()
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            .service(actix_files::Files::new("/api/images", &public_dir))
            .configure(handlers::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
