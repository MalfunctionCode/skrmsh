use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, FromRow, Clone)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub team_id: Option<Uuid>,
    pub is_team_owner: bool,
    pub created_at: OffsetDateTime
}

#[derive(Debug, Serialize)]
pub struct PlayerResponse {
    pub id: Uuid,
    pub username: String,
    team_id: Option<Uuid>,
    is_team_owner: bool,
    created_at: OffsetDateTime
}

#[derive(Debug, Deserialize)]
pub struct CreatePlayerRequest {
    pub username: String,
    pub password: String,
}

impl From<Player> for PlayerResponse {
    fn from(player: Player) -> Self {
        Self {
            id: player.id,
            username: player.username,
            team_id: player.team_id,
            is_team_owner: player.is_team_owner,
            created_at: player.created_at,
        }
    }
}
