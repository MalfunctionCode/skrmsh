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
pub struct SingleTeamResponse {
    pub id: Uuid,
    pub name: String,
    pub owner: PlayerResponse,
    pub players: Vec<PlayerResponse>,
}

#[derive(Debug, Serialize)]
pub struct TeamListResponse{
    pub id: Uuid,
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct TeamPlayerModificationRequest {
    pub player_id: Uuid,
    pub team_id: Uuid
}

#[derive(Debug, Deserialize)]
pub struct CreateTeamRequest{
    pub name: String
}

impl From<Team> for TeamListResponse {
    fn from(team: Team) -> Self{

        Self{
            id:team.id,
            name:team.name,
        }
    }
}
