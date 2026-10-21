use axum::{Router, routing::{get,post,put}};

use crate::AppState;
use crate::services::player_service::*;
use crate::services::team_service::*;

pub fn players_router() -> Router<AppState>{
    Router::new()
        .route("/players", get(get_players))
        .route("/players/{id}", get(get_player))
        .route("/players", post(create_player))
        .route("/players/verify", post(verify_player))
        .route("/players/me", get(get_me))
        .route("/players/logout", post(logout))
}

pub fn teams_router() -> Router<AppState>{
    Router::new()
        .route("/teams", get(get_teams))
        .route("/teams/{id}", get(get_team))
        .route("/teams", post(create_team))
        .route("/teams/add", put(add_player_to_team))
        .route("/teams/set_captain", put(set_player_as_captain))
}