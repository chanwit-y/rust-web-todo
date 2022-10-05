use crate::model::db::init_db;

use super::TodoMac;


#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {

	let db = init_db().await?;

	let todos = TodoMac::list(&db).await?;

	assert_eq!(2, todos.len());

	Ok({})
}