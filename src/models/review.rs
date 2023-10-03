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

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ReviewWithUser {
    pub id: i64,
    pub author_id: String,
    pub camp_id: i64,
    #[serde(with = "ts_seconds")]
    pub ctime: sqlx::types::chrono::DateTime<Utc>,
    pub title: String,
    pub body: String,
    pub rating: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
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

        update_calc_review_average(camp_id, db).await?;

        Ok(review)
    }

    pub async fn delete(db: &PgPool, _utx: UserCtx, review_id: i64) -> Result<String, Error> {
        let deleted_review = sqlx::query_as!(
            Review,
            "DELETE FROM reviews where id = $1 returning *",
            review_id
        )
        .fetch_one(db)
        .await?;

        update_calc_review_average(deleted_review.camp_id, db).await?;

        Ok("Review deleted successfully".to_string())
    }

    pub async fn get_camp_reviews(db: &PgPool, camp_id: i64) -> Result<Vec<ReviewWithUser>, Error> {
        let reviews = sqlx::query_as::<_, ReviewWithUser>(
            "SELECT * FROM reviews JOIN users ON author_id = supabase_id where camp_id = $1",
        )
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

async fn update_calc_review_average(camp_id: i64, db: &PgPool) -> Result<(), Error> {
    let camp_reviews = sqlx::query!("SELECT (rating) FROM reviews WHERE camp_id = $1", camp_id)
        .fetch_all(db)
        .await?;

    let total_reviews = camp_reviews.len() as i32;

    let avg =
        camp_reviews.iter().map(|review| review.rating).sum::<i32>() as f32 / total_reviews as f32;

    sqlx::query!("UPDATE camps SET rating = $1 WHERE id = $2", avg, camp_id)
        .execute(db)
        .await?;

    Ok(())
}
