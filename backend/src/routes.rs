use axum::{Router, routing::{get,post}};

use crate::AppState;
use crate::services::player_service::{create_player, get_player, get_players, verify_player};

pub fn players_router() -> Router<AppState>{
    Router::new()
        .route("/players", get(get_players))
        .route("/players/{id}", get(get_player))
        .route("/players", post(create_player))
        .route("/players/verify", post(verify_player))
}