use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use uuid::Uuid;
use crate::error::AppError;
use crate::models::player::{CreatePlayerRequest, Player, PlayerResponse};
use crate::AppState;
use crate::auth::{generate_jwt, verify_jwt};

pub async fn get_players(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let players = sqlx::query_as!(Player, "SELECT * FROM players")
        .fetch_all(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("no players found".into()),
            _ => AppError::Database(e),
        })?;

    let response: Vec<PlayerResponse> = players.into_iter().map(PlayerResponse::from).collect();
    Ok(Json(response))
}

pub async fn get_player(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let player = sqlx::query_as!(Player, "SELECT * FROM players WHERE id = $1", id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("player not found".into()),
            _ => AppError::Database(e),
        })?;

    Ok(Json(PlayerResponse::from(player)))
}

pub async fn create_player(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(req): Json<CreatePlayerRequest>,
) -> Result<impl IntoResponse, AppError> {
    let password_hash = Argon2::default()
        .hash_password(req.password.as_bytes())
        .map_err(|_| AppError::Internal("failed to hash password".into()))?
        .to_string();

    let player = sqlx::query_as!(
        Player,
        "INSERT INTO players (id, password_hash, username)
         VALUES ($1, $2, $3)
         RETURNING id, password_hash, username, team_id, is_team_owner, created_at",
        Uuid::new_v4(),
        password_hash,
        req.username
    )
        .fetch_one(&state.db)
        .await?;

    let cookie = Cookie::build(("token", generate_jwt(player.id)?))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::days(7))
        .build();

    Ok((jar.add(cookie), Json(PlayerResponse::from(player))))
}

pub async fn verify_player(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(req): Json<CreatePlayerRequest>,
) -> Result<impl IntoResponse, AppError> {
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM players WHERE username = $1",
        req.username
    )
        .fetch_one(&state.db)
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let parsed_hash = PasswordHash::new(&player.password_hash)
        .map_err(|_| AppError::Internal("invalid stored hash".into()))?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)?;

    let cookie = Cookie::build(("token", generate_jwt(player.id)?))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::days(7))
        .build();

    Ok((jar.add(cookie), Json(PlayerResponse::from(player))))
}

pub async fn get_me(State(state): State<AppState>,
jar:CookieJar) -> Result<impl IntoResponse, AppError> {
    let token = jar.get("token").ok_or(AppError::Unauthorized)?.value();

    let player_id = verify_jwt(token)?;

    let player = sqlx::query_as!(Player, "SELECT * FROM players WHERE id = $1", player_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| AppError::Unauthorized)?;

    Ok(Json(PlayerResponse::from(player)))
}

pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    jar.remove(Cookie::build("token").path("/"))
}