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
            body: String::new(),
            rating: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewPatch {
    pub body: String,
    pub rating: i32,
    pub photos: Option<Vec<String>>,
}

pub struct ReviewManager;

impl ReviewManager {
    pub async fn create(
        db: &PgPool,
        utx: UserCtx,
        data: ReviewPatch,
        camp_id: i64,
    ) -> Result<Review, Error> {
        Self::delete_user_reviews_for_camp(db, &utx, camp_id).await?;

        let review = sqlx::query_as!(Review, "INSERT INTO reviews (camp_id, author_id, body, rating) VALUES ($1, $2, $3, $4) returning *",
    camp_id, utx.clone().user_id, &data.body, &data.rating)

            .fetch_one(db)
            .await?;

        update_calc_review_average(camp_id, db).await?;

        Ok(review)
    }

    pub async fn get_review(
        db: &PgPool,
        utx: &UserCtx,
        camp_id: i64,
    ) -> Result<Option<Review>, Error> {
        let review = sqlx::query_as!(
            Review,
            "SELECT * FROM reviews WHERE camp_id = $1 AND author_id = $2",
            camp_id,
            utx.user_id
        )
        .fetch_optional(db)
        .await?;

        Ok(review)
    }

    pub async fn delete(db: &PgPool, _utx: &UserCtx, review_id: i64) -> Result<String, Error> {
        sqlx::query!("DELETE FROM reviews where id = $1", review_id,)
            .execute(db)
            .await?;

        update_calc_review_average(review_id, db).await?;

        Ok("Review deleted successfully".to_string())
    }

    pub async fn delete_user_reviews_for_camp(
        db: &PgPool,
        utx: &UserCtx,
        camp_id: i64,
    ) -> Result<String, Error> {
        sqlx::query!(
            "DELETE FROM reviews where camp_id = $1 AND author_id = $2",
            camp_id,
            utx.user_id
        )
        .execute(db)
        .await?;

        update_calc_review_average(camp_id, db).await?;

        Ok("Reviews deleted successfully".to_string())
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
