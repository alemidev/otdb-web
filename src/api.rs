use serde::Deserialize;

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

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionDifficulty {
	Easy,
	Medium,
	Hard,
}

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(Deserialize, serde::Serialize)]
pub struct ApiCategoriesList {
	pub trivia_categories: Vec<ApiCategory>,
}

#[derive(Deserialize, serde::Serialize)]
pub struct ApiCategory {
	pub id: u32,
	pub name: String,
}

#[derive(serde::Deserialize)]
pub struct ApiResponse {
	pub response_code: ApiResponseCode,
	pub results: Vec<ApiResponseQuestion>,
}

#[derive(serde::Deserialize)]
pub struct ApiResponseQuestion {
	#[serde(rename = "type")]
	pub kind: QuestionType,
	pub difficulty: QuestionDifficulty,
	pub category: String,
	pub question: String,
	pub correct_answer: String,
	pub incorrect_answers: Vec<String>,
}

#[derive(serde_repr::Deserialize_repr, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ApiResponseCode {
	/// Returned results successfully.
	Success = 0,
	/// Could not return results. The API doesn't have enough questions for your query. (Ex. Asking for 50 Questions in a Category that only has 20.)
	NoResults = 1,
	/// Contains an invalid parameter. Arguements passed in aren't valid. (Ex. Amount = Five)
	InvalidParameter = 2,
	/// Session Token does not exist.
	TokenNotFound = 3,
	/// Session Token has returned all possible questions for the specified query. Resetting the Token is necessary.
	TokenEmpty = 4,
	/// Too many requests have occurred. Each IP can only access the API once every 5 seconds.
	RateLimit = 5,
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

		assert_eq!(
			json,
			r#"{"amount":10,"token":"asd","command":"request","category":1,"difficulty":"medium","type":"multiple","encode":"base64"}"#
		);
	}

	#[test]
	fn assert_response_code_deserializes_as_expected() {
		let response = r#"{
			"response_code": 0,
			"results": [
				{
					"type": "multiple",
					"difficulty": "easy",
					"category": "Science: Computers",
					"question": "The C programming language was created by this American computer scientist. ",
					"correct_answer": "Dennis Ritchie",
					"incorrect_answers":[
						"Tim Berners Lee",
						"al-Khwarizmi",
						"Willis Ware"
					]
				}
			]
		}"#;

		let parsed: super::ApiResponse =
			serde_json::from_str(response).expect("failed deserializing response");

		assert_eq!(parsed.response_code, super::ApiResponseCode::Success);
	}
}
