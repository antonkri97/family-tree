use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::Local;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::{google_oauth::GoogleUserResult, model::User};

pub async fn user_exists(pool: &PgPool, email: &str) -> Result<bool, Error> {
    let exist: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)")
            .bind(email.to_string())
            .fetch_one(pool)
            .await?;

    Ok(exist.unwrap_or(false))
}

pub async fn create_user(
    pool: PgPool,
    email: &str,
    password: &str,
    name: &str,
) -> Result<User, Error> {
    let user = User {
        id: None,
        name: name.to_owned(),
        verified: false,
        email: email.to_owned().to_lowercase(),
        provider: "local".to_string(),
        role: "user".to_string(),
        password: hash(password.to_string(), DEFAULT_COST).unwrap(),
        photo: "default.png".to_string(),
        created_at: None,
        updated_at: None,
    };

    sqlx::query_as!(
        User,
        "
        INSERT INTO users (name, email, password, role, photo, verified, provider)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
    ",
        user.name.clone(),
        user.email.clone(),
        user.password.clone(),
        user.role.clone(),
        user.photo.clone(),
        user.verified as bool,
        user.provider.clone(),
    )
    .execute(&pool)
    .await?;

    Ok(user)
}

// /// Поиск пользователя по логину
// pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
//     let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
//         .fetch_optional(pool)
//         .await?;

//     Ok(user)
// }

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email,)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn get_user_by_email_and_password(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = match get_user_by_email(pool, email).await? {
        Some(user) if verify(password, &user.password).unwrap_or(false) => Some(user),
        Some(_) => None,
        None => None,
    };

    Ok(user)
}

pub async fn update_google_user(
    pool: &PgPool,
    id: &str,
    email: &str,
    photo: &str,
) -> Result<User, sqlx::Error> {
    let user_id = Uuid::parse_str(id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

    sqlx::query!(
        r#"
        UPDATE users
        SET email = $1, photo = $2, updated_at = $3
        WHERE id = $4
        "#,
        email,
        photo,
        Local::now().naive_local(),
        user_id
    )
    .execute(pool)
    .await?;

    let updated_user = sqlx::query_as!(
        User,
        r#"
        SELECT *
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(updated_user)
}

pub async fn insert_google_user(
    pool: &PgPool,
    id: Uuid,
    google_user: GoogleUserResult,
) -> Result<User, sqlx::Error> {
    let datetime = Local::now().naive_local();
    let user_data = User {
        id: Some(id),
        name: google_user.name,
        verified: google_user.verified_email,
        email: google_user.email,
        provider: "Google".to_string(),
        role: "user".to_string(),
        password: "".to_string(),
        photo: google_user.picture,
        created_at: Some(datetime),
        updated_at: Some(datetime),
    };

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (
            id, name, email, photo, verified, provider, role, password, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
        user_data.id,
        user_data.name,
        user_data.email,
        user_data.photo,
        user_data.verified,
        user_data.provider,
        user_data.role,
        user_data.password,
        user_data.created_at,
        user_data.updated_at
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: &str) -> Result<Option<User>, sqlx::Error> {
    // Преобразуем строку `user_id` в Uuid
    let user_uuid = Uuid::parse_str(user_id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

    // Выполняем SQL-запрос для поиска пользователя по id
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT *
        FROM users
        WHERE id = $1
        "#,
        user_uuid
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
