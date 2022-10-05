use super::init_db;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let result = sqlx::query("SELECT * from todo WHERE id in (100, 101)").fetch_all(&db).await?;
    assert_eq!(2, result.len(), "number of seed todos");

    Ok(())
}
