use crate::otdb::ApiCategoriesList;
use dashmap::DashMap;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::watch;
use uuid::{Uuid};

#[derive(Clone)]
pub struct GameState {
	pub game_state_inner: Arc<GameStateInner>,
}

impl GameState {
	pub fn new(categories: ApiCategoriesList) -> Self {
		Self {
			game_state_inner: Arc::new(GameStateInner {
				categories,
				clients: DashMap::new(),
			}),
		}
	}

	/// Register a new client and return id, rx
	pub fn register_client(
		&self,
		initial_opts: GameOptions,
	) -> (Uuid, watch::Receiver<GameOptions>) {
		let (tx, rx) = watch::channel(initial_opts);
		let id = Uuid::new_v4();
		self.game_state_inner.clients.insert(id, tx);
		(id, rx)
	}

	/// Removes a disconnected client
	pub fn unregister_client(&self, id: &Uuid) {
		self.game_state_inner.clients.remove(id);
	}

	/// Broadcasts an update to all clients
	pub fn broadcast(&self, new_opts: GameOptions) {
		for entry in self.game_state_inner.clients.iter() {
			let _ = entry.value().send_replace(new_opts.clone());
		}
	}
}

pub struct GameStateInner {
	pub categories: crate::otdb::ApiCategoriesList,
	pub clients: DashMap<uuid::Uuid, watch::Sender<GameOptions>>,
}

pub struct GameSession {
	pub code: String,
	pub players: HashMap<String, Player>,
	pub options: GameOptions,
	pub questions: Vec<super::otdb::ApiResponseQuestion>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GameOptions {
	pub amount: u32,
	pub category: Option<u32>,
	pub difficulty: Option<super::otdb::QuestionDifficulty>,
	pub kind: Option<super::otdb::QuestionType>,
}

pub struct Player {
	pub name: String,
	pub correct: u32,
	pub score: u32,
}
