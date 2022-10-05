use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{fs, path::PathBuf};

const PG_HOST: &str = "localhost";
const PG_DB: &str = "pg_demo";
const PG_USER: &str = "postgres";
const PG_PWD: &str = "postgres";

const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
    // {
    //     let root_db = new_db_pool(PG_HOST, PG_DB, PG_USER, PG_PWD, 1).await?;
    //     pexec(&root_db, SQL_RECREATE).await?;
    // }

    // let app_db = new_db_pool(PG_HOST, PG_DB, PG_USER, PG_PWD, 1).await?;
    // let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
    //     .into_iter()
    //     .filter_map(|e| e.ok().map(|e| e.path()))
    //     .collect();

    // paths.sort();

    // for path in paths {
    //     if let Some(path) = path.to_str() {
    //         // if path.ends_with(".sql") && path != SQL_RECREATE {
    //         if path.ends_with(".sql")  {
    //             pexec(&app_db, &path).await?;
    //         }
    //     }
    // }

    new_db_pool(PG_HOST, PG_DB, PG_USER, PG_PWD, 1).await
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    // Read the file
    let content = fs::read_to_string(file).map_err(|ex| {
        println!("Error reading {} (cause: {:?})", file, ex);
        ex
    })?;

    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        match sqlx::query(&sql).execute(db).await {
            Ok(_) => (),
            Err(ex) => println!("Warning - pexec - Sql file '{}' Failed cause: {}", file, ex),
        }
    }

    Ok(())
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
