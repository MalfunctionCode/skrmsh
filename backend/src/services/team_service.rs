use crate::AppState;
use crate::error::AppError;
use crate::models::player::{Player, PlayerResponse};
use crate::models::team::{
    CreateTeamRequest, SingleTeamResponse, Team, TeamListResponse, TeamPlayerModificationRequest,
};
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use hyper::StatusCode;
use uuid::Uuid;

pub async fn get_teams(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let teams = sqlx::query_as!(Team, "SELECT * FROM teams")
        .fetch_all(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("no teams found".into()),
            _ => AppError::Database(e),
        })?;
    let response: Vec<TeamListResponse> = teams.into_iter().map(TeamListResponse::from).collect();

    Ok(Json(response))
}

pub async fn get_team(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let team = sqlx::query_as!(Team, "SELECT * FROM teams WHERE id = $1", id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("team not found".into()),
            _ => AppError::Internal(e.to_string()),
        })?;

    let players = sqlx::query_as!(Player, "SELECT * FROM players WHERE team_id = $1", id)
        .fetch_all(&state.db)
        .await?;

    let owner = players
        .iter()
        .find(|p| p.is_team_owner)
        .ok_or(AppError::NotFound("owner not found".into()))?
        .clone();

    let response = SingleTeamResponse {
        id: team.id,
        name: team.name,
        owner: PlayerResponse::from(owner),
        players: players.into_iter().map(PlayerResponse::from).collect(),
    };

    Ok(Json(response))
}

pub async fn create_team(
    State(state): State<AppState>,
    Json(req): Json<CreateTeamRequest>,
) -> Result<impl IntoResponse, AppError> {
    let team = sqlx::query_as!(
        Team,
        "INSERT INTO teams (id, name) VALUES ($1, $2) RETURNING *",
        Uuid::new_v4(),
        req.name
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::Database(e))?;

    Ok((StatusCode::CREATED, Json(TeamListResponse::from(team))))
}

pub async fn add_player_to_team(
    State(state): State<AppState>,
    Json(req): Json<TeamPlayerModificationRequest>,
) -> Result<impl IntoResponse, AppError> {
    sqlx::query_as!(
        Player,
        "UPDATE players SET team_id = $1 WHERE id = $2",
        req.team_id,
        req.player_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound("no player found".into()),
        _ => AppError::Database(e),
    });

    Ok(StatusCode::OK)
}

pub async fn set_player_as_captain(
    State(state): State<AppState>,
    Json(req): Json<TeamPlayerModificationRequest>,
) -> Result<impl IntoResponse, AppError> {
    sqlx::query_as!(
        Player,
        "UPDATE players SET is_team_owner = true WHERE id = $1 AND team_id = $2",
        req.player_id,
        req.team_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound("no player found".into()),
        _ => AppError::Database(e),
    });

    Ok(StatusCode::OK)
}
