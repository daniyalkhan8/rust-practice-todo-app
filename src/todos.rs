use std::result;
use sqlx::PgPool;
use sqlx::types::time::OffsetDateTime;
use crate::UpdateTodo;

#[derive(Debug)]
pub struct Todo {
    id: i32,
    title: String,
    done: Option<bool>,
    created_at: OffsetDateTime,
}

pub async fn add_todo(pool: &PgPool, title: String) -> anyhow::Result<i32> {
    let rec = sqlx::query!(
        r#"INSERT INTO todo (title) VALUES ($1) RETURNING id"#, title
    ).fetch_one(pool).await?;

    Ok(rec.id)
}

pub async fn get_todo(pool: &PgPool, id: i32) -> anyhow::Result<Todo> {
    let todo = sqlx::query_as!(
        Todo,
        r#"SELECT * FROM todo WHERE id=$1"#, id
    ).fetch_one(pool).await?;

    Ok(todo)
}

pub async fn list_todos(pool: &PgPool) -> anyhow::Result<Vec<Todo>> {
    let todos = sqlx::query_as!(
        Todo,
        r#"SELECT * from todo ORDER BY id"#
    ).fetch_all(pool).await?;

    Ok(todos)
}

pub async fn update_todo(pool: &PgPool, update_todo: &UpdateTodo) -> anyhow::Result<()> {
    let result = sqlx::query!(
        r#"UPDATE todo SET title = $1 WHERE id = $2"#, update_todo.title, update_todo.id
    ).execute(pool).await?;

    if result.rows_affected() == 0 {
        anyhow::bail!("Todo with ID: {} not found", update_todo.id);
    }

    Ok(())
}

pub async fn mark_as_done(pool: &PgPool, id: &i32) -> anyhow::Result<()> {
    let result = sqlx::query!(
        r#"UPDATE todo SET done = true WHERE id = $1"#, id
    ).execute(pool).await?;

    if result.rows_affected() == 0 {
        anyhow::bail!("Todo with ID: {} not found", id);
    }

    Ok(())
}

pub async fn delete_todo(pool: &PgPool, id: &i32) -> anyhow::Result<()> {
    let result = sqlx::query!(
        r#"DELETE FROM todo WHERE id = $1"#, id
    ).execute(pool).await?;

    if result.rows_affected() == 0 {
        anyhow::bail!("Todo with ID: {} not found", id);
    }

    Ok(())
}
