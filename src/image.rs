use sqlx::Database;
use tokio::{fs::File, io::AsyncWriteExt};
use warp::Filter;

use crate::db::Image;

pub fn images<'a, E>(
    db: E,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres> + Clone,
{
    get_image_by_hash(db.clone()).or(upload_image(db))
}

fn get_image_by_hash<'a, E>(
    db: E,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres> + Clone,
{
    warp::path!("images" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(|hash, db| async {
            handlers::get_image_by_hash(hash, db)
                .await
                .map_err::<warp::Rejection, _>(|e| e.into())
        })
}

fn upload_image<'a, E>(
    db: E,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres> + Clone,
{
    warp::path!("images")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 1024 * 10)) // 10MiB
        .and(warp::body::bytes())
        .and(with_db(db))
        .and_then(|body: bytes::Bytes, db| async {
            handlers::upload_image(body, db)
                .await
                .map_err::<warp::Rejection, _>(|e| e.into())
        })
}

fn with_db<'a, E>(db: E) -> impl Filter<Extract = (E,), Error = std::convert::Infallible> + Clone
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres> + Clone,
{
    warp::any().map(move || db.clone())
}

mod handlers {
    use tokio::{fs::File, io::AsyncWriteExt};

    use crate::{db::Image, Result};

    pub async fn upload_image<'a, E>(body: impl AsRef<[u8]>, db: E) -> Result<impl warp::Reply>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let hash = seahash::hash(body.as_ref());
        let file_path = format!("cache/{}", hash);
        let mut file = File::create(file_path).await?;
        file.write_all(body.as_ref()).await?;

        Ok(warp::reply())
    }

    pub async fn get_image_by_hash<'a, E>(hash: impl ToString, db: E) -> Result<impl warp::Reply>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let image = Image::get_by_hash(hash, db).await?;

        Ok(warp::reply::json(&image))
    }
}
