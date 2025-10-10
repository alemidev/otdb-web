mod error;

mod categories;
mod questions;

use axum::{Router, routing::get};

pub async fn serve(addr: String, state: crate::game::GameState) -> Result<(), std::io::Error> {
	tracing::info!("serving backend on {addr}");

	let app = Router::new()
		.route("/categories", get(categories::get))
		//.route("/questions", post(get_questions))
		.with_state(state);

	let listener = tokio::net::TcpListener::bind(addr).await?;
	axum::serve(listener, app)
		.with_graceful_shutdown(shutdown())
		.await?;

	tracing::info!("shutting down backend");

	Ok(())
}

async fn shutdown() {
	if let Err(e) = tokio::signal::ctrl_c().await {
		tracing::error!("error waiting for ctrl-c signal: {e}");
	};
}
