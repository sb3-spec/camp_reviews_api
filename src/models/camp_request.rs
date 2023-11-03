#![allow(unused)]
use crate::auth::UserCtx;

use super::{CampPatch, Error};
use serde_derive::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CampRequest {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub phone_number: String,
    pub street_address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip_code: String,
    pub email: String,
    pub tags: Option<Vec<String>>,
    pub image_urls: Option<Vec<String>>,
    pub website: Option<String>,
    pub apt_suite_other: Option<String>,
    pub user_id: String,
}

pub struct CampRequestManager;

impl CampRequestManager {
    pub async fn new_request(
        db: &PgPool,
        utx: &UserCtx,
        data: CampPatch,
    ) -> Result<CampRequest, Error> {
        let camp_request = sqlx::query_as!(
            CampRequest,
            "insert into camp_requests (name, description, phone_number, street_address, city, state, country, email, zip_code, website, tags, image_urls, user_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) returning *",
            data.name.unwrap_or_default(),
            data.description.unwrap_or("".to_string()),
            data.phone_number.unwrap_or_default(),
            data.street_address.unwrap_or_default(),
            data.city.unwrap_or("".to_string()),
            data.state.unwrap_or("".to_string()),
            data.country.unwrap_or("".to_string()),
            data.email.unwrap_or_default(),
            data.zip_code.unwrap_or_default(),
            data.website.unwrap_or_default(),
            &data.tags.unwrap_or_default(),
            &data.image_urls.unwrap_or_default(),
            utx.user_id,
        ).fetch_one(db).await?;

        Ok(camp_request)
    }

    pub async fn delete_camp_request(
        db: &PgPool,
        _utx: &UserCtx,
        camp_request_id: i64,
    ) -> Result<(), Error> {
        sqlx::query!("DELETE FROM camp_requests WHERE id = $1", camp_request_id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn get_camp_request(db: &PgPool, camp_request_id: i64) -> Result<CampRequest, Error> {
        let camp_request = sqlx::query_as!(
            CampRequest,
            "SELECT * FROM camp_requests WHERE id = $1",
            camp_request_id
        )
        .fetch_one(db)
        .await?;

        Ok(camp_request)
    }

    pub async fn get_camp_requests(db: &PgPool, _utx: UserCtx) -> Result<Vec<CampRequest>, Error> {
        let camp_requests = sqlx::query_as!(CampRequest, "SELECT * FROM camp_requests")
            .fetch_all(db)
            .await?;

        Ok(camp_requests)
    }
}
