use std::env;
use axum::{Router, routing::get};
use sqlx::{PgPool};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use crate::routes::players_router;

mod models;
mod services;
pub mod error;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
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

    let app_state = AppState { db: pool };

    let app = Router::new()
        .route("/", get(hello_world))
        .merge(players_router())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}