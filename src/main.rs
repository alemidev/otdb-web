mod api;
mod game;

use crate::api::{API_CATEGORIES, ApiCategoriesList, API_PATH};
use axum::{Json, Router, response::IntoResponse, routing::get,extract::{State}};
use std::sync::Arc;
use reqwest::Url;
use tokio::sync::RwLock;
use crate::game::GameOptions;

struct AppState {
	categories: RwLock<ApiCategoriesList>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	let categories = match fetch_categories().await {
		Ok(list) => {
			println!("Loaded {} categories.", list.trivia_categories.len());
			list
		}
		Err(err) => {
			eprintln!("Failed to fetch categories: {err}");
			std::process::exit(1);
		}
	};

	let app_state = Arc::new(AppState {
		categories: RwLock::new(categories),
	});

	let app = Router::new()
		.route("/categories", get(get_categories))
		//.route("/questions", post(get_questions))
		.with_state(app_state);

	let addr = "0.0.0.0:80";
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	println!("Server running on http://{}", addr);
	axum::serve(listener, app).await.unwrap();

	Ok(())
}

async fn fetch_categories() -> Result<ApiCategoriesList, reqwest::Error> {
	let resp = reqwest::get(API_CATEGORIES)
		.await?
		.error_for_status()?
		.json::<ApiCategoriesList>()
		.await?;
	Ok(resp)
}

async fn get_categories(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let categories = state.categories.read().await;
	Json(categories.clone())
}
async fn get_questions(
	State(_): State<Arc<AppState>>,
	Json(options): Json<GameOptions>,
){
	let mut url = Url::parse(API_PATH).unwrap();
	{
		let mut query = url.query_pairs_mut();
		query.append_pair("amount", &options.amount.to_string());

		if let Some(cat) = options.category {
			query.append_pair("category", &cat.to_string());
		}

		if let Some(diff) = &options.difficulty {
			let diff_str = match diff {
				api::QuestionDifficulty::Easy => "easy",
				api::QuestionDifficulty::Medium => "medium",
				api::QuestionDifficulty::Hard => "hard",
			};
			query.append_pair("difficulty", diff_str);
		}

		if let Some(kind) = &options.kind {
			let kind_str = match kind {
				api::QuestionType::Multiple => "multiple",
				api::QuestionType::Boolean => "boolean",
			};
			query.append_pair("type", kind_str);
		}
	}

	println!("Request to: {url}");

	let response = reqwest::get(url).await;
}