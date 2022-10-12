use sqlb::HasFields;

use super::db::Db;
use crate::{model, security::UserCtx};

// region: Todo Types
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64,
    pub title: String,
}

#[derive(sqlb::Fields, Default, Debug, Clone)]
pub struct TodoPatch {
    pub cid: Option<i64>,
    pub title: Option<String>,
    pub status: Option<TodoStatus>,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Open,
    Clone,
}
sqlb::bindable!(TodoStatus);
// endregion: Todo Types

// region: TodoMac
pub struct TodoMac;

impl TodoMac {
    const TABLE: &'static str = "todo";
    const COLUMNS: &'static [&'static str] = &["id", "cid", "title", "status"];
}

impl TodoMac {
    pub async fn create(db: &Db, utx: &UserCtx, data: TodoPatch) -> Result<Todo, model::Error> {
        // let sql = "insert into todo (cid, title) values ($1,$2) returning id, cid, title";
        // let query = sqlx::query_as::<_, Todo>(&sql)
        //     .bind(123 as i64) // Fixme - should come from user context
        //     .bind(data.title.unwrap_or_else(|| "untitled".to_string()));
        // let todo = query.fetch_one(db).await?;

        let mut fields = data.fields();
        fields.push(("cid", 123).into());

        let sb = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(Self::COLUMNS);

        let todo = sb.fetch_one(db).await?;

        Ok(todo)
    }

    pub async fn get(db: &Db, _utx: &UserCtx, id: i64) -> Result<Todo, model::Error> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .and_where_eq("id", id);

        let todo = sb.fetch_one(db).await?;

        Ok(todo)
    }

    pub async fn update(
        db: &Db,
        _utx: &UserCtx,
        id: i64,
        data: TodoPatch,
    ) -> Result<Todo, model::Error> {

        let sb = sqlb::update()
            .table(Self::TABLE)
            .data(data.fields())
            .and_where_eq("id", id)
            .returning(Self::COLUMNS);

        let todo = sb.fetch_one(db).await?;

        Ok(todo)
    }

    pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<Todo>, model::Error> {
        // let sql = "SELECT id, cid, title FROM todo ORDER BY id desc";

        // let query = sqlx::query_as(&sql);
        // let todos = query.fetch_all(db).await?;

        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .order_by("!id");

        let todos = sb.fetch_all(db).await?;

        Ok(todos)
    }
}
// endregion:  TodoMac

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
