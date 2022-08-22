mod db;
mod image;


type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error: `{0}`")]
    Sqlx(#[from] sqlx::Error),
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("unknown error")]
    Unknown
}

impl warp::reject::Reject for Error {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = sqlx::postgres::PgPool::connect("postgresql://postgres:notalivepassword@localhost").await?;
    sqlx::migrate!().run(&pool).await?;

    println!("Hello, world!");

    Ok(())
}
