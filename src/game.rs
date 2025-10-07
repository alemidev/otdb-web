use std::collections::HashMap;

pub struct GameSession {
	pub code: String,
	pub players: HashMap<String, Player>,
	pub options: GameOptions,
	pub questions: Vec<super::api::ApiResponseQuestion>,
}

pub struct GameOptions {
	pub amount: u32,
	pub category: Option<u32>,
	pub difficulty: Option<super::api::QuestionDifficulty>,
	pub kind: Option<super::api::QuestionType>,
}

pub struct Player {
	pub name: String,
	pub correct: u32,
	pub score: u32,
}
