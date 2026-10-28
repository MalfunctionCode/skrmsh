use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct Hero{
    pub id: i16,
    pub slug: String,
    pub name: String
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "hero_tier", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum HeroTier{
    Main,
    Comfortable,
    Learning,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TierlistEntry{
    pub hero_id: i16,
    pub tier: HeroTier
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTierlistRequest {
    pub entries: Vec<TierlistEntry>,
}
