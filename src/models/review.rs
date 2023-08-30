use chrono::{serde::ts_seconds, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use super::Error;
use crate::auth::UserCtx;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Review {
    pub id: i64,
    pub author_id: String,
    pub camp_id: i64,
    #[serde(with = "ts_seconds")]
    pub ctime: sqlx::types::chrono::DateTime<Utc>,
    pub title: String,
    pub body: String,
    pub rating: i32,
}

impl Default for Review {
    fn default() -> Self {
        Self {
            id: 0,
            author_id: String::new(),
            camp_id: 0,
            ctime: sqlx::types::chrono::DateTime::default(),
            title: String::new(),
            body: String::new(),
            rating: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewPatch {
    pub title: Option<String>,
    pub body: Option<String>,
    pub rating: Option<i32>,
}

pub struct ReviewManager;

impl ReviewManager {
    pub async fn create(
        db: &PgPool,
        utx: UserCtx,
        data: ReviewPatch,
        camp_id: i64,
    ) -> Result<Review, Error> {
        let query = "INSERT INTO reviews (camp_id, author_id, title, body, rating) VALUES ($1, $2, $3, $4, $5) returning *";

        let review = sqlx::query_as::<_, Review>(query)
            .bind(camp_id)
            .bind(utx.user_id)
            .bind(&data.title.unwrap_or_default())
            .bind(&data.body.unwrap_or_default())
            .bind(&data.rating.unwrap_or_default())
            .fetch_one(db)
            .await?;

        Ok(review)
    }

    pub async fn delete(db: &PgPool, _utx: UserCtx, review_id: i64) -> Result<String, Error> {
        let query = "DELETE FROM reviews where id = $1 returning *";

        sqlx::query(query).bind(review_id).execute(db).await?;

        Ok("Review deleted successfully".to_string())
    }

    pub async fn get_camp_reviews(db: &PgPool, camp_id: i64) -> Result<Vec<Review>, Error> {
        let reviews = sqlx::query_as::<_, Review>("SELECT * FROM reviews where camp_id = $1")
            .bind(camp_id)
            .fetch_all(db)
            .await?;

        Ok(reviews)
    }

    pub async fn delete_all_camp_reviews(
        db: &PgPool,
        _utx: UserCtx,
        camp_id: i64,
    ) -> Result<(), Error> {
        let query = "DELETE FROM reviews where camp_id = $1";

        sqlx::query(query).bind(camp_id).execute(db).await?;

        Ok(())
    }
}
