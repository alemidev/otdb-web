mod api;
mod game;

use crate::api::{API_CATEGORIES, ApiCategoriesList};
use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	let app_state = Arc::new(AppState {});

	let app = Router::new()
		.route("/categories", get(get_categories))
		.with_state(app_state);

	let addr = "0.0.0.0:80";
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	println!("Server running on http://{}", addr);
	axum::serve(listener, app).await.unwrap();

	Ok(())
}

async fn get_categories(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
	match reqwest::get(API_CATEGORIES).await {
		Ok(resp) => match resp.error_for_status() {
			Ok(valid_resp) => match valid_resp.json::<ApiCategoriesList>().await {
				Ok(categories) => Json(categories).into_response(),
				Err(err) => (
					axum::http::StatusCode::INTERNAL_SERVER_ERROR,
					format!("Error while parsing JSON: {err}"),
				)
					.into_response(),
			},
			Err(err) => (
				axum::http::StatusCode::BAD_GATEWAY,
				format!("Error in the remote response: {err}"),
			)
				.into_response(),
		},
		Err(err) => (
			axum::http::StatusCode::BAD_GATEWAY,
			format!("Error during the request: {err}"),
		)
			.into_response(),
	}
}
