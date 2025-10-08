use axum::{http::StatusCode, response::IntoResponse};

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
	#[error("error fetching data from opentriviadb: {0}")]
	Fetch(#[from] reqwest::Error),
}

impl IntoResponse for ApiError {
	fn into_response(self) -> axum::response::Response {
		let msg = self.to_string();
		let code = match self {
			Self::Fetch(_) => StatusCode::INTERNAL_SERVER_ERROR,
		};

		(
			code,
			axum::Json(serde_json::json!({
				"error": true,
				"message": msg,
			})),
		)
			.into_response()
	}
}
