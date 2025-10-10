use std::sync::Arc;
use axum::extract::{State, WebSocketUpgrade};
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;
use clap::Parser;
use crate::game::GameState;
use crate::settings::manage_pre_lobby;

mod api;
mod game;
mod otdb;
mod settings;


/// otdb-web -- online multiplayer trivia game, powered by opentriviadb
#[derive(Parser)]
struct Cli {
	/// address to bind backend on
	#[arg(long, short, default_value = "127.0.0.1:8040")]
	addr: String,

	/// number of worker threads to spawn for the runtime
	#[arg(long, default_value_t = 4)]
	threads: usize,

	/// enable debug logging
	#[arg(long, default_value_t = false)]
	debug: bool,
}

fn main() {
	let args = Cli::parse();

	//let (tx, _rx) = watch::channel(());

	tracing_subscriber::fmt()
		.compact()
		.with_max_level(if args.debug {
			tracing::Level::DEBUG
		} else {
			tracing::Level::INFO
		})
		.init();

	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_io()
		.worker_threads(args.threads)
		.thread_name("otdb-web-worker")
		.build()
		.expect("could not build runtime");

	if let Err(e) = rt.block_on(run(args)) {
		tracing::error!("exception running otdb-web: {e}");
		std::process::exit(1);
	}
}

async fn run(args: Cli) -> Result<(), Box<dyn std::error::Error>> {
	let categories: otdb::ApiCategoriesList = reqwest::get(otdb::API_CATEGORIES_PATH)
		.await?
		.error_for_status()?
		.json()
		.await?;

	let state = game::GameState::new(categories);

	api::serve(args.addr, state).await?;

	Ok(())
}

async fn ws_handler(
	ws: WebSocketUpgrade,
	State(state): State<Arc<GameState>>,
) -> impl IntoResponse {
	ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<GameState>) {
	manage_pre_lobby(socket, state).await;
	//TODO other stuff
}


