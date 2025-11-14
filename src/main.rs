mod board;
mod controller;
use crate::controller::{generate_board, solve_board};
use axum::{routing::post, Router};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Initializing server...");

    let app = Router::new()
        .route("/generate", post(generate_board))
        .route("/solve", post(solve_board));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
