use crate::board::Board;
use axum::Json;
use http::StatusCode;
use serde::Deserialize;

pub async fn generate_board(
    Json(payload): Json<GenerateBoardPayload>,
) -> (StatusCode, Json<Board>) {
    let board = Board::generate_new_sudoku(payload.size, payload.clues);
    (StatusCode::OK, Json(board))
}

#[derive(Deserialize)]
pub struct GenerateBoardPayload {
    size: usize,
    clues: usize,
}
