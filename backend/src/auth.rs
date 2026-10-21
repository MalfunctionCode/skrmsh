use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::AppError;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

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