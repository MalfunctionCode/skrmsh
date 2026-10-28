use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, FromRow, Clone)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub team_id: Option<Uuid>,
    pub is_team_owner: bool,
    pub twitch_link: Option<String>,
    pub discord: Option<String>,
    pub created_at: OffsetDateTime
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PlayerResponse {
    pub id: Uuid,
    pub username: String,
    pub team_id: Option<Uuid>,
    pub is_team_owner: bool,
    pub twitch_link: Option<String>,
    pub discord: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime
}

#[derive(Debug, Deserialize, ToSchema)]
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
            twitch_link: player.twitch_link,
            discord: player.discord,
            is_team_owner: player.is_team_owner,
            created_at: player.created_at,
        }
    }
}
