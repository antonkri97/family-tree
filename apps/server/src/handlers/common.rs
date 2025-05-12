use actix_web::{HttpResponse, Responder, get};

#[get("/healthchecker")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "How to Implement Google OAuth2 in Rust";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}
