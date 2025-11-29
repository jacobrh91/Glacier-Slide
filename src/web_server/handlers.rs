use crate::{board, game_state::GameConfig};

use super::metrics::next_request_id;

use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct BoardRequest {
    pub difficulty: String,
}

pub async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

pub async fn board_handler(Query(params): Query<BoardRequest>) -> impl IntoResponse {
    let request_id = next_request_id();
    tracing::info!(
        request_id,
        difficulty = %params.difficulty,
        "Received board generation request",
    );

    let config = match GameConfig::for_server_from_difficulty(&params.difficulty) {
        Ok(cfg) => cfg,
        Err(msg) => {
            tracing::warn!(request_id, error = %msg, "Bad difficulty parameter");
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "request_id": request_id, "error": msg })),
            );
        }
    };

    let board = match board::Board::generate_solvable_board(&config, Some(request_id)) {
        Ok(b) => b,
        Err(e) => {
            tracing::error!(
                request_id,
                error = %e,
                "Failed to generate solvable board"
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "request_id": request_id,
                    "error": "failed to generate solvable board"
                })),
            );
        }
    };

    let json_str = board.get_layout_json();

    match serde_json::from_str::<serde_json::Value>(&json_str) {
        Ok(value) => {
            tracing::info!(request_id, "Successfully generated board");
            (
                StatusCode::OK,
                Json(json!({ "request_id": request_id, "board": value })),
            )
        }
        Err(e) => {
            tracing::error!(
                request_id,
                error = %e,
                "Failed to serialize board JSON"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "request_id": request_id,
                    "error": "failed to serialize board"
                })),
            )
        }
    }
}
