use axum::http::{header, Method};
use std::io;
use std::sync::{Arc, Mutex};
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../errors.rs"]
mod errors;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;


use routers::*;
use state::AppState;

#[tokio::main]
async fn main() -> io::Result<()>
{
    dotenv().ok(); 

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
    let shared_state = Arc::new(AppState
    {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<header::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .max_age(std::time::Duration::from_secs(3600));

    let app = app_router(shared_state).layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app)
        .await
        .map_err(io::Error::other)
}
