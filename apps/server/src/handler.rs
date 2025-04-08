use crate::{
    authenticate_token::AuthenticationGuard,
    google_oauth::{get_google_user, request_token},
    model::{AppState, LoginUserSchema, QueryCode, RegisterUserSchema, TokenClaims, User},
    response::{FilteredUser, UserData, UserResponse},
    user_repo::{
        create_user, get_user_by_email, get_user_by_email_and_password, get_user_by_id,
        insert_google_user, update_google_user, user_exists,
    },
};
use actix_web::http::header::LOCATION;
use actix_web::{
    HttpResponse, Responder,
    cookie::{Cookie, time::Duration as ActixWebDuration},
    get, post, web,
};
use chrono::{Duration, prelude::*};
use jsonwebtoken::{EncodingKey, Header, encode};
use uuid::Uuid;

pub fn user_to_response(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.clone(),
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        verified: user.verified.to_owned(),
        photo: user.photo.to_owned(),
        provider: user.provider.to_owned(),
        role: user.role.to_owned(),
        created_at: user.created_at.clone(),
        updated_at: user.updated_at.clone(),
    }
}

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "How to Implement Google OAuth2 in Rust";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let pool = data.pool.clone();

    if let Ok(true) = user_exists(&pool, &body.email).await {
        return HttpResponse::Conflict()
            .json(serde_json::json!({"status": "fail","message": "Email already exist"}));
    }

    match create_user(pool, &body.email, &body.password, &body.name).await {
        Ok(user) => {
            let json_response = UserResponse {
                status: "success".to_string(),
                data: UserData {
                    user: user_to_response(&user),
                },
            };
            HttpResponse::Ok().json(json_response)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Failed to register user",
            "info": e.to_string()
        })),
    }
}

#[post("/auth/login")]
async fn login_user_handler(
    body: web::Json<LoginUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let pool = data.pool.clone();

    let user = match get_user_by_email_and_password(&pool, &body.email, &body.password).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::BadRequest().json(
                serde_json::json!({"status": "fail", "message": "Invalid email or password"}),
            );
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": "Failed to register user",
                "info": e.to_string()
            }));
        }
    };

    if user.provider == "Google" {
        return HttpResponse::Unauthorized()
            .json(serde_json::json!({"status": "fail", "message": "Use Google OAuth2 instead"}));
    }

    let jwt_secret = data.env.jwt_secret.to_owned();
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(data.env.jwt_max_age)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.unwrap().to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(ActixWebDuration::new(60 * data.env.jwt_max_age, 0))
        .http_only(true)
        .finish();

    println!("{}", cookie);

    HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({"status": "success", "user": user_to_response(&user)}))
}

#[get("/sessions/oauth/google")]
async fn google_oauth_handler(
    query: web::Query<QueryCode>,
    data: web::Data<AppState>,
) -> impl Responder {
    let code = &query.code;
    let state = &query.state;

    if code.is_empty() {
        return HttpResponse::Unauthorized().json(
            serde_json::json!({"status": "fail", "message": "Authorization code not provided!"}),
        );
    }

    let token_response = request_token(code.as_str(), &data).await;
    if token_response.is_err() {
        let message = token_response.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let token_response = token_response.unwrap();
    let google_user = get_google_user(&token_response.access_token, &token_response.id_token).await;
    if google_user.is_err() {
        let message = google_user.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let google_user = google_user.unwrap();

    let pool = data.pool.clone();
    let email = google_user.email.to_lowercase();
    let user = match get_user_by_email(&pool, &email).await {
        Ok(Some(user)) => Some(user),
        Ok(None) => None,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "info": e.to_string()
            }));
        }
    };

    let user_id: String;

    if let Some(user) = user {
        user_id = user.id.unwrap().to_string();

        if let Err(e) = update_google_user(&pool, &user_id, &email, &google_user.picture).await {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "info": e.to_string()
            }));
        }
    } else {
        let id = Uuid::new_v4();
        user_id = id.to_owned().to_string();

        if let Err(e) = insert_google_user(&pool, id, google_user).await {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "info": e.to_string()
            }));
        }
    }

    let jwt_secret = data.env.jwt_secret.to_owned();
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(data.env.jwt_max_age)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user_id,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(ActixWebDuration::new(60 * data.env.jwt_max_age, 0))
        .http_only(true)
        .finish();

    let frontend_origin = data.env.client_origin.to_owned();
    let mut response = HttpResponse::Found();

    response.append_header((LOCATION, format!("{}{}", frontend_origin, state)));
    response.cookie(cookie);
    response.finish()
}

#[get("/auth/logout")]
async fn logout_handler(_: AuthenticationGuard) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({"status": "success"}))
}

#[get("/users/me")]
async fn get_me_handler(
    auth_guard: AuthenticationGuard,
    data: web::Data<AppState>,
) -> impl Responder {
    match get_user_by_id(&data.pool, &auth_guard.user_id).await {
        Ok(Some(user)) => HttpResponse::Ok()
            .json(serde_json::json!({"status": "success", "user": user_to_response(&user)})),
        Ok(None) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "failed", "err": "can not find such user"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "info": e.to_string()
        })),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(register_user_handler)
        .service(login_user_handler)
        .service(google_oauth_handler)
        .service(logout_handler)
        .service(get_me_handler);

    conf.service(scope);
}
