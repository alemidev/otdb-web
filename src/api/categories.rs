use axum::extract::{Json, State};

use super::error::ApiResult;


pub async fn get(State(state): State<crate::game::GameState>) -> ApiResult<Json<crate::otdb::ApiCategoriesList>> {
	tracing::debug!("client requested category list");
	Ok(Json(state.0.categories.clone()))
}
