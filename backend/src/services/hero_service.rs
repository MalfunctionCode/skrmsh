use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use crate::AppState;
use crate::error::AppError;
use crate::models::hero::Hero;

pub async fn get_heroes(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let heroes = sqlx::query_as!(Hero, "SELECT * FROM heroes")
        .fetch_all(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("no heroes found".into()),
            _ => AppError::Database(e),
        })?;

    Ok(Json(heroes))
}