use std::error::Error;

use crate::model::{AppState, QueryCode, TokenClaims};
use crate::repo::{get_user_by_email, insert_google_user, update_google_user};
use actix_web::http::header::LOCATION;
use actix_web::{
    HttpResponse, Responder,
    cookie::{Cookie, time::Duration as ActixWebDuration},
    get, web,
};
use chrono::{Duration, prelude::*};
use jsonwebtoken::{EncodingKey, Header, encode};
use reqwest::{Client, Url};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Deserialize)]
pub struct GoogleUserResult {
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub picture: String,
}

pub async fn request_token(
    authorization_code: &str,
    data: &web::Data<AppState>,
) -> Result<OAuthResponse, Box<dyn Error>> {
    let redirect_url = data.env.google_oauth_redirect_url.to_owned();
    let client_secret = data.env.google_oauth_client_secret.to_owned();
    let client_id = data.env.google_oauth_client_id.to_owned();

    let root_url = "https://oauth2.googleapis.com/token";
    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_url.as_str()),
        ("client_id", client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", client_secret.as_str()),
    ];
    let response = client.post(root_url).form(&params).send().await?;

    if response.status().is_success() {
        let oauth_response = response.json::<OAuthResponse>().await?;
        Ok(oauth_response)
    } else {
        let message = "An error occurred while trying to retrieve access token.";
        Err(From::from(message))
    }
}

pub async fn get_google_user(
    access_token: &str,
    id_token: &str,
) -> Result<GoogleUserResult, Box<dyn Error>> {
    let client = Client::new();
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
    url.query_pairs_mut().append_pair("alt", "json");
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let response = client.get(url).bearer_auth(id_token).send().await?;

    if response.status().is_success() {
        let user_info = response.json::<GoogleUserResult>().await?;
        Ok(user_info)
    } else {
        let message = "An error occurred while trying to retrieve user information.";
        Err(From::from(message))
    }
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
