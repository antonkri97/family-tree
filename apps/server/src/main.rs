mod authenticate_token;
mod config;
mod google_oauth;
mod handler;
mod model;
mod response;
mod user_repo;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, http::header, web};
use config::Config;
use dotenv::dotenv;
use model::AppState;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = Config::init().database_url.to_string();
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let db = AppState::init(pool);
    let app_data = web::Data::new(db);
    let public_dir = std::env::current_dir().unwrap().join("public");

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            .service(actix_files::Files::new("/api/images", &public_dir))
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
