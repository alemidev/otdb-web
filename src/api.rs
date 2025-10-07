
pub const API_PATH: &str = "https://opentdb.com/api.php";
pub const API_CATEGORIES: &str = "https://opentdb.com/api_category.php";

#[derive(serde::Serialize)]
pub struct ApiQuery {
	pub amount: Option<u32>,
	pub token: Option<String>,
	pub command: Option<ApiCommand>,
	pub category: Option<u32>,
	pub difficulty: Option<QuestionDifficulty>,
	#[serde(rename = "type")]
	pub kind: Option<QuestionType>,
	pub encode: Option<ApiEncoding>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiCommand {
	Request,
	Reset,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionDifficulty {
	Easy,
	Medium,
	Hard,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
	Multiple,
	Boolean,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiEncoding {
	UrlLegacy,
	Url3986,
	Base64,
}



#[derive(serde::Deserialize)]
pub struct ApiCategoriesList {
	pub trivia_categories: Vec<ApiCategory>,
}

#[derive(serde::Deserialize)]
pub struct ApiCategory {
	pub id: u32,
	pub name: String,
}

#[cfg(test)]
mod test {

	#[test]
	fn assert_serializes_as_expected() {
		let q = super::ApiQuery {
			amount: Some(10),
			token: Some("asd".to_string()),
			command: Some(super::ApiCommand::Request),
			category: Some(1),
			difficulty: Some(super::QuestionDifficulty::Medium),
			kind: Some(super::QuestionType::Multiple),
			encode: Some(super::ApiEncoding::Base64),
		};

		let json = serde_json::to_string(&q).expect("failed serializing query");

		assert_eq!(json, r#"{"amount":10,"token":"asd","command":"request","category":1,"difficulty":"medium","type":"multiple","encode":"base64"}"#);
	}
}
