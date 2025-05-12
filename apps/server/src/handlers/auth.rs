use crate::{
    handlers::model::{UserData, UserResponse},
    model::{AppState, LoginUserSchema, RegisterUserSchema, TokenClaims, User},
    repo::{create_user, get_user_by_email_and_password, get_user_by_id, user_exists},
};
use actix_web::{
    FromRequest, HttpRequest,
    dev::Payload,
    error::{Error as ActixWebError, ErrorUnauthorized},
    http::{self},
};
use actix_web::{
    HttpResponse, Responder,
    cookie::{Cookie, time::Duration as ActixWebDuration},
    get, post, web,
};
use chrono::{Duration, prelude::*};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde_json::json;
use std::future::{Ready, ready};

use super::model::FilteredUser;

pub struct AuthenticationGuard {
    pub user_id: String,
}

impl FromRequest for AuthenticationGuard {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "You are not logged in, please provide token"}),
            )));
        }

        let data = req.app_data::<web::Data<AppState>>().unwrap();

        let jwt_secret = data.env.jwt_secret.to_owned();
        let decode = decode::<TokenClaims>(
            token.unwrap().as_str(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => ready(Ok(AuthenticationGuard {
                user_id: token.claims.sub,
            })),
            Err(_) => ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "Invalid token or usre doesn't exists"}),
            ))),
        }
    }
}

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
            return HttpResponse::Unauthorized().json(
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
