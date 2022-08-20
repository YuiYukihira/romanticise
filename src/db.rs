use sqlx::{self, FromRow, query};

type Result<T> = sqlx::Result<T>;

#[derive(Debug)]
pub struct Image {
    pub id: i32,
    pub image_hash: String,
    pub image_path: String,
}


impl Image {
    pub async fn insert<'a, E>(&self, conn: E) -> Result<i32>
    where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let rec = query!(
            r#"
INSERT INTO Image (id, image_hash, image_path)
VALUES ($1, $2, $3)
RETURNING id;
            "#,
            self.id,
            self.image_hash,
            self.image_path
        ).fetch_one(conn).await?;

        Ok(rec.id)
    }
}
