use std::env;
use axum::{routing::get, Json, Router};
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use sqlx::{PgPool};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use utoipa::OpenApi;
use crate::routes::{players_router, teams_router};
use crate::models::team::*;
use crate::models::player::*;

mod models;
mod services;
pub mod error;
pub mod routes;
pub mod auth;

#[derive(OpenApi)]
#[openapi(
    components(
        schemas(
            PlayerResponse,
            CreatePlayerRequest,
            SingleTeamResponse,
            TeamListResponse,
            TeamPlayerModificationRequest,
            CreateTeamRequest
        )
    )
)]
struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    db: PgPool
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_connection_string).await
        .expect("Couldn't connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app_state = AppState { db: pool};

    let api_routes = Router::new()
        .route("/", get(hello_world))
        .merge(players_router())
        .merge(teams_router())
        .route("/api-docs/openapi.json", get(openapi_schema))
        .with_state(app_state);

    let app = Router::new().nest("/api", api_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn openapi_schema() -> impl IntoResponse{
    Json(ApiDoc::openapi())
}
async fn hello_world() -> &'static str {
    "Hello, world!"
}