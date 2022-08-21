use anyhow;
use tokio;


mod db;
mod image;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = sqlx::postgres::PgPool::connect("postgresql://postgres:notalivepassword@localhost").await?;
    sqlx::migrate!().run(&pool).await?;

    println!("Hello, world!");

    Ok(())
}
