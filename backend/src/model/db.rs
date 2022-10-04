use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

const PG_HOST: &str = "localhost";
const PG_DB: &str = "pg_demo";
const PG_USER: &str = "postgres";
const PG_PWD: &str = "postgres";

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
    new_db_pool(PG_HOST, PG_DB, PG_USER, PG_PWD, 1).await
}

async fn new_db_pool(
    host: &str,
    db: &str,
    user: &str,
    pwd: &str,
    max_con: u32,
) -> Result<Db, sqlx::Error> {
    let con_str = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connections(max_con)
        // .connect_timeout(Duration::from_millis(500))
        .connect(&con_str)
        .await
}

#[cfg(test)]
mod tests {
    use super::init_db;

    #[tokio::test]
    async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
        let db = init_db().await?;

	let result = sqlx::query("SELECT * from todo").fetch_all(&db).await?;
	assert_eq!(2, result.len(), "number of seed todos");

	Ok(())
    }
}
