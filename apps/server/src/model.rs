use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::config;

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub provider: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct AppState {
    pub env: config::Config,
    pub pool: Pool<Postgres>,
}

impl AppState {
    pub fn init(p: Pool<Postgres>) -> AppState {
        AppState {
            env: config::Config::init(),
            pool: p,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}
