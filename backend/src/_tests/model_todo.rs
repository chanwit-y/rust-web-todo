use crate::{model::db::init_db, security::utx_from_token};

use super::{TodoMac, TodoPatch};

#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    let data_fx = TodoPatch {
        title: Some("test - model_todo_create 1".to_string()),
        ..Default::default()
    };

    let utx = utx_from_token("123").await?;
    let todo_created = TodoMac::create(&db, &utx, data_fx.clone()).await?;
    println!("\n\n->> {:?}", todo_created);
    assert!(todo_created.id >= 1000, "Id should be >= 1000");
    assert_eq!(data_fx.title.unwrap(), todo_created.title);

    Ok(())
}

#[tokio::test]
async fn model_todo_get_ok() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let id = 100;
    let utx = utx_from_token("123").await?;
    let todo = TodoMac::get(&db, &utx, id).await?;

    assert!(todo.id == id, "todos id should be = 100");

    Ok(())
}

#[tokio::test]
async fn model_todo_get_wong_id() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let utx = utx_from_token("123").await?;
    let result = TodoMac::get(&db, &utx, 999).await;

    print!("\n --> {:?}", result);

    Ok(())
}

#[tokio::test]
async fn model_todo_update() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let data_fx = TodoPatch {
        title: Some("test - model_todo_update 1000".to_string()),
        ..Default::default()
    };
    let utx = utx_from_token("123").await?;
    let todo = TodoMac::update(&db, &utx, 1000, data_fx).await?;

    assert!(todo.title == "test - model_todo_update 1000", "todo title should be 'test - model_todo_update 1000'");

    Ok(())
}

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let utx = utx_from_token("123").await?;
    let todos = TodoMac::list(&db, &utx).await?;

    assert!(todos.len() > 0, "todos len should be > 0");
//     assert_eq!(2, todos.len());
//     println!("\n\n->> {:?}", todos);
    // todo 101
//     assert_eq!(101, todos[0].id);
//     assert_eq!(123, todos[0].cid);
    // todo 100
//     assert_eq!(100, todos[1].id);
//     assert_eq!(123, todos[1].cid);
//     assert_eq!("todo 100", todos[1].title);

    Ok({})
}
