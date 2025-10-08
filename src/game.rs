use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct GameState(pub Arc<GameStateInner>);

impl GameState {
	pub fn new(categories: crate::otdb::ApiCategoriesList) -> Self {
		Self(Arc::new(GameStateInner { categories }))
	}
}

pub struct GameStateInner {
	pub categories: crate::otdb::ApiCategoriesList,
}



pub struct GameSession {
	pub code: String,
	pub players: HashMap<String, Player>,
	pub options: GameOptions,
	pub questions: Vec<super::otdb::ApiResponseQuestion>,
}

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
