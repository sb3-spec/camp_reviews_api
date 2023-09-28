use super::Error;
use super::Review;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::auth::UserCtx;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub supabase_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}

pub struct UserManager;

impl UserManager {
    pub async fn new_user(db: &PgPool, data: User, _utx: UserCtx) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (supabase_id, first_name, last_name, email, username) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            data.supabase_id,
            data.first_name,
            data.last_name,
            data.email,
            data.username
        )
      .fetch_one(db).await?;

        Ok(user)
    }

    pub async fn get_user(db: &PgPool, utx: UserCtx) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE supabase_id = $1",
            utx.user_id
        )
        .fetch_one(db)
        .await?;

        Ok(user)
    }

    pub async fn delete_user(db: &PgPool, utx: UserCtx) -> Result<(), Error> {
        sqlx::query!("DELETE FROM users WHERE supabase_id = $1", utx.user_id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn get_user_reviews(db: &PgPool, utx: UserCtx) -> Result<Vec<Review>, Error> {
        let reviews = sqlx::query_as!(
            Review,
            "SELECT * FROM reviews WHERE author_id = $1",
            utx.user_id
        )
        .fetch_all(db)
        .await?;

        Ok(reviews)
    }
}
