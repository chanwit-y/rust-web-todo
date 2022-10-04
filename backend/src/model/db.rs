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
#[path = "../_tests/model_db.rs"]
mod tests;
