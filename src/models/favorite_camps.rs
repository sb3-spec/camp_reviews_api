use serde_derive::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::auth::UserCtx;

use super::Error;

#[derive(Debug, Serialize, Deserialize, Default)]

pub struct UserCampJunction {
    pub user_id: String,
    pub camp_id: i64,
}

pub struct UserCampJunctionManager;

impl UserCampJunctionManager {
    pub async fn query(
        db: &PgPool,
        utx: UserCtx,
        camp_id: i64,
    ) -> Result<Vec<UserCampJunction>, Error> {
        let camp_user_junctions = sqlx::query_as!(
            UserCampJunction,
            "SELECT * FROM users_camps WHERE (user_id = $1) AND (camp_id = $2)",
            utx.user_id,
            camp_id
        )
        .fetch_all(db)
        .await?;

        Ok(camp_user_junctions)
    }

    pub async fn _qyery_by_camp_id(
        db: &PgPool,
        camp_id: i64,
    ) -> Result<Vec<UserCampJunction>, Error> {
        let camp_user_junctions = sqlx::query_as!(
            UserCampJunction,
            "SELECT * FROM users_camps WHERE camp_id = $1",
            camp_id
        )
        .fetch_all(db)
        .await?;

        Ok(camp_user_junctions)
    }

    pub async fn qyery_by_user_id(
        db: &PgPool,
        user_id: String,
    ) -> Result<Vec<UserCampJunction>, Error> {
        let camp_user_junctions = sqlx::query_as!(
            UserCampJunction,
            "SELECT * FROM users_camps WHERE user_id = $1",
            user_id
        )
        .fetch_all(db)
        .await?;

        Ok(camp_user_junctions)
    }

    pub async fn favorite(db: &PgPool, utx: UserCtx, camp_id: i64) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO users_camps (user_id, camp_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            utx.user_id,
            camp_id
        )
        .execute(db)
        .await?;

        Ok(())
    }
    pub async fn unfavorite(db: &PgPool, utx: UserCtx, camp_id: i64) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM users_camps WHERE (user_id, camp_id) = ($1, $2)",
            utx.user_id,
            camp_id
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
