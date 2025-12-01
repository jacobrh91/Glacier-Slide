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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Query;
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use http_body_util::BodyExt;
    use serde_json::Value;

    // Helper to turn any IntoResponse into (StatusCode, JSON body)
    async fn status_and_json(res: impl IntoResponse) -> (StatusCode, Value) {
        let response: Response = res.into_response();
        let status = response.status();

        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        (status, body)
    }

    #[tokio::test]
    async fn health_handler_returns_ok_status_and_json() {
        let (status, body) = status_and_json(health_handler().await).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["status"], "ok");
    }

    #[tokio::test]
    async fn board_handler_rejects_unknown_difficulty() {
        let req = BoardRequest {
            difficulty: "insane".to_string(),
        };

        let (status, body) = status_and_json(board_handler(Query(req)).await).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);

        let err = body["error"].as_str().unwrap_or("");
        assert!(
            err.contains("Unknown difficulty"),
            "unexpected error message: {err}"
        );
    }

    // This test exercises the happy-path with a valid difficulty.
    // Since it is an easy level, the test should still run quickly.
    #[tokio::test]
    async fn board_handler_accepts_valid_difficulty_and_returns_board() {
        let req = BoardRequest {
            difficulty: "easy".to_string(),
        };

        let (status, body) = status_and_json(board_handler(Query(req)).await).await;

        assert_eq!(status, StatusCode::OK);

        // Should contain a numeric request_id
        assert!(
            body.get("request_id").and_then(Value::as_u64).is_some(),
            "expected numeric request_id in response: {body:?}"
        );

        // Should contain a `board` object
        assert!(
            body.get("board").is_some(),
            "expected board field in response: {body:?}"
        );
    }
}
