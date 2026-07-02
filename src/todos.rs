use sqlx::PgPool;
use sqlx::types::time::OffsetDateTime;

#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
    created_at: OffsetDateTime
}

pub async fn add_todo(pool: &PgPool, title: String) -> anyhow::Result<i32> {
    let rec = sqlx::query!(
        r#"INSERT INTO todo (title) VALUES ($1) RETURNING id"#, title
    ).fetch_one(pool).await?;

    Ok(rec.id)
}
