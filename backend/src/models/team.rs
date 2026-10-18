use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::models::player::{Player, PlayerResponse};

#[derive(Debug, FromRow)]
pub struct Team{
    pub id: Uuid,
    pub name: String,
    pub created_at: OffsetDateTime
}

#[derive(Debug, Serialize)]
pub struct TeamResponse {
    pub id: Uuid,
    pub name: String,
    pub owner: PlayerResponse,
    pub players: Vec<PlayerResponse>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTeamRequest{
    pub name: String
}