use sqlx::PgPool;
use sqlx::types::time::OffsetDateTime;

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

pub async fn get_todo(pool: &PgPool, id: u32) -> anyhow::Result<Todo> {
    let todo = sqlx::query_as!(
        Todo,
        r#"SELECT * FROM todo WHERE id=$1"#, id as i32
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
