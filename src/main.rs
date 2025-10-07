mod api;
mod game;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	let categories: api::ApiCategoriesList = reqwest::get(api::API_CATEGORIES)
		.await?
		.error_for_status()?
		.json()
		.await?;

	for category in categories.trivia_categories {
		println!("{} : {}", category.id, category.name);
	}

	Ok(())
}
