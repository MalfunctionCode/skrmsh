use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;
use crate::error::AppError;
use crate::models::player::Player;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

pub struct AuthPlayer(pub Player);

fn secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn generate_jwt(player_id: Uuid) -> Result<String, AppError> {
    let exp = (time::OffsetDateTime::now_utc() + time::Duration::days(7)).unix_timestamp() as usize;
    encode(
        &Header::default(),
        &Claims { sub: player_id, exp },
        &EncodingKey::from_secret(secret().as_bytes()),
    )
        .map_err(|_| AppError::Internal("failed to create token".into()))
}

pub fn verify_jwt(token: &str) -> Result<Uuid, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret().as_bytes()),
        &Validation::default(),
    )
        .map(|data| data.claims.sub)
        .map_err(|_| AppError::Unauthorized)
}


impl FromRequestParts<AppState> for AuthPlayer {
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let token = jar.get("token").ok_or(AppError::Unauthorized)?.value();

        let player_id = verify_jwt(token)?;

        let player = sqlx::query_as!(Player, "SELECT * FROM players WHERE id = $1", player_id)
            .fetch_one(&state.db)
            .await
            .map_err(|_| AppError::Unauthorized)?;

        Ok(AuthPlayer(player))
    }
}