use sqlx::Database;
use warp::Filter;


use crate::db::Image;



pub fn images<'a, E>(db: E) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
E: sqlx::Executor<'a, Database = sqlx::Postgres> + Clone,
{
    get_image_by_hash(db.clone())
        .or(upload_image(db))
}

fn get_image_by_hash<'a, E>(db: E) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
E: sqlx::Executor<'a, Database = sqlx::Postgres> + Clone,
{
    warp::path!("images" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(|hash, db| async {
            Ok::<_, std::convert::Infallible>(warp::reply())
        })
}

fn upload_image<'a, E>(db: E) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
E: sqlx::Executor<'a, Database = sqlx::Postgres> + Clone,
{
    warp::path!("images")
        .and(warp::post())
        .and(with_db(db))
        .and_then(|db| async {
            Ok::<_, std::convert::Infallible>(warp::reply())
        })
}




fn with_db<'a, E>(db: E) -> impl Filter<Extract = (E,), Error = std::convert::Infallible> + Clone
where
E: sqlx::Executor<'a, Database = sqlx::Postgres> + Clone,
{
    warp::any().map(move || db.clone())
}
