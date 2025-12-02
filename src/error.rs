use axum::{
	response::{IntoResponse, Response},
	Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
	BadRequest(String),
	Internal(String),
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		let (status, message) = match self {
			AppError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
			AppError::Internal(msg) => {
				tracing::error!("Internal error: {}", msg);
				(axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Server error".into())
			}
		};

		let body = Json(json!({
			"error": message
		}));

		(status, body).into_response()
	}
}
// Error handling - to be implemented
