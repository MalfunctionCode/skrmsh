use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, phc::{PasswordHash}},
    Argon2
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::error::AppError;
use crate::models::player::{CreatePlayerRequest, Player};

pub async fn create_player(pool: &PgPool, req: CreatePlayerRequest) -> Result<Player, AppError>{
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(req.password.as_bytes()).map_err(|_| AppError::Internal("failed to hash password".into()))?.to_string();

    let player = sqlx::query_as!(
        Player,
        "INSERT INTO players (id, password_hash, username)
         VALUES ($1, $2, $3)
         RETURNING id, password_hash, username, team_id, is_team_owner, created_at",
        Uuid::new_v4(),
        password_hash,
        req.username
    )
        .fetch_one(pool)
        .await?;

    Ok(player)
}

pub async fn verify_user(
    pool: &PgPool,
    username: String,
    password: String,
) -> Result<Player, AppError> {
    let user = sqlx::query_as!(
        Player,
        "SELECT * FROM players WHERE username = $1",
        username
    )
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Unauthorized)?;  // don't reveal if username exists

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::Internal("invalid stored hash".into()))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)?;

    Ok(user)
}