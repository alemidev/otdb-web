use axum::extract::Json;

use super::error::ApiResult;

pub async fn get(
	Json(options): Json<crate::game::GameOptions>,
) -> ApiResult<Json<crate::otdb::ApiResponseQuestion>> {
	// TODO have a global client, don't build one for each request
	let question = reqwest::Client::default()
		.get(crate::otdb::API_PATH)
		.query(&crate::otdb::ApiQuery {
			difficulty: options.difficulty,
			amount: Some(options.amount),
			category: options.category,
			kind: options.kind,
			token: None,
			command: None,
			encode: None,
		})
		.send()
		.await?
		.json()
		.await?;

	Ok(Json(question))
}
