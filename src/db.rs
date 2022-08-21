use sqlx::{self, FromRow, query, query_as};

pub type Result<T> = sqlx::Result<T>;

#[derive(Debug)]
pub struct Image {
    pub id: i32,
    pub image_hash: String,
    pub image_path: String,
}

#[derive(Debug)]
pub struct Tag {
    pub id: i32,
    pub description: String,
}


impl Image {
    pub async fn get_by_hash<'a, E>(hash: impl ToString, conn: E) -> Result<Self>
    where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let rec = query_as!(Image,
            r#"
SELECT * FROM Image
WHERE image_hash = $1;
            "#,
            hash.to_string()).fetch_one(conn).await?;

        Ok(rec)
    }

    pub async fn insert<'a, E>(image_hash: impl ToString, image_path: impl ToString, conn: E) -> Result<i32>
    where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let rec = query!(
            r#"
INSERT INTO Image (image_hash, image_path)
VALUES ($1, $2)
RETURNING id;
            "#,
            image_hash.to_string(),
            image_path.to_string()
        ).fetch_one(conn).await?;

        Ok(rec.id)
    }

    pub async fn tags<'a, E>(&self, conn: E) -> Result<impl warp::Stream<Item = Result<Tag>> + 'a>
    where
    E: sqlx::Executor<'a, Database = sqlx::Postgres> + 'a,
    {
        let recs = query_as!(Tag,
            r#"
SELECT Tag.* FROM Tag
INNER JOIN ImageTag
ON ImageTag.tag_id = Tag.id
WHERE ImageTag.image_id = $1;
            "#, self.id).fetch(conn);

        Ok(recs)
    }
}
