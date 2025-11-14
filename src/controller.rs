use crate::board::Board;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GenerateBoardPayload {
    size: usize,
    clues: usize,
}

pub async fn generate_board(Json(payload): Json<GenerateBoardPayload>) -> impl IntoResponse {
    let board = Board::generate_new_sudoku(payload.size, payload.clues);
    (StatusCode::OK, Json(board)).into_response()
}

pub async fn solve_board(Json(mut payload): Json<Board>) -> impl IntoResponse {
    if payload.solve_brute_force() {
        (StatusCode::OK, Json(payload)).into_response()
    } else {
        (StatusCode::BAD_REQUEST, Json("No solution found")).into_response()
    }
}
