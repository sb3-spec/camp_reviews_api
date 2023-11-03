use super::{
    camp_request::{CampRequest, CampRequestManager},
    Error, Review,
};
use crate::auth::UserCtx;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct Camp {
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
    pub rating: Option<f32>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CampPatch {
    pub name: Option<String>,
    pub description: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub zip_code: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub tags: Option<Vec<String>>,
    pub apt_suite_other: Option<String>,
    pub image_urls: Option<Vec<String>>,
}

pub struct CampManager;

impl CampManager {
    pub async fn get_all_camps(db: &PgPool, _utx: UserCtx) -> Result<Vec<Camp>, Error> {
        let query = "SELECT * FROM camps";
        let all_camps = sqlx::query_as::<_, Camp>(query).fetch_all(db).await?;

        Ok(all_camps)
    }

    pub async fn add_camp(db: &PgPool, _utx: UserCtx, camp_request_id: i64) -> Result<Camp, Error> {
        let data = CampRequestManager::get_camp_request(db, camp_request_id).await?;

        let camp: Camp = sqlx::query_as!(
            Camp,
            "insert into camps (name, description, phone_number, street_address, city, state, country, email, zip_code, website, tags, image_urls) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) returning *",
            data.name,
            data.description,
            data.phone_number,
            data.street_address,
            data.city,
            data.state,
            data.country,
            data.email,
            data.zip_code,
            data.website.unwrap_or_default(),
            &data.tags.unwrap_or_default(),
            &data.image_urls.unwrap_or_default(),
        ).fetch_one(db).await?;

        Ok(camp)
    }

    pub async fn get_camp(db: &PgPool, id: i64, _utx: UserCtx) -> Result<Camp, Error> {
        let camp = sqlx::query_as!(Camp, "SELECT * FROM camps WHERE id = $1", id)
            .fetch_one(db)
            .await?;

        Ok(camp)
    }

    pub async fn get_featured_camps(db: &PgPool) -> Result<Vec<Camp>, Error> {
        let featured_camps =
            sqlx::query_as!(Camp, "SELECT * FROM camps ORDER BY rating DESC LIMIT 10")
                .fetch_all(db)
                .await?;

        Ok(featured_camps)
    }

    pub async fn update_camp(
        db: &PgPool,
        id: i64,
        data: CampPatch,
        _utx: UserCtx,
    ) -> Result<Camp, Error> {
        let original_camp = sqlx::query_as!(Camp, "SELECT * FROM camps WHERE id = $1", id)
            .fetch_one(db)
            .await?;

        let camp = sqlx::query_as!(Camp, "UPDATE camps SET name=$1, description=$2, phone_number=$3, street_address=$4, city=$5, state=$6, country=$7, email=$8, zip_code=$9, website=$10, tags=$11, image_urls=$12 WHERE id = $13 returning *",
            data.name.unwrap_or(original_camp.name),
            data.description.unwrap_or(original_camp.description),
            data.phone_number.unwrap_or(original_camp.phone_number),
            data.street_address.unwrap_or(original_camp.street_address),
            data.city.unwrap_or(original_camp.city),
            data.state.unwrap_or(original_camp.state),
            data.country.unwrap_or(original_camp.country),
            data.email.unwrap_or(original_camp.email),
            data.zip_code.unwrap_or(original_camp.zip_code),
            Some(data.website.unwrap_or(original_camp.website.unwrap_or_default())),
            &data.tags.unwrap_or(original_camp.tags.unwrap_or_default()),
            &data.image_urls.unwrap_or(original_camp.image_urls.unwrap_or_default()),
            id).fetch_one(db).await?;

        Ok(camp)
    }

    pub async fn delete_camp(db: &PgPool, id: i64, _utx: UserCtx) -> Result<Camp, Error> {
        let camp = sqlx::query_as!(Camp, "DELETE FROM camps WHERE id = $1 returning *", id)
            .fetch_one(db)
            .await?;

        Ok(camp)
    }

    pub async fn get_camp_reviews(
        db: &PgPool,
        camp_id: i64,
        _utx: UserCtx,
    ) -> Result<Vec<Review>, Error> {
        let reviews = sqlx::query_as!(Review, "SELECT * FROM reviews WHERE camp_id = $1", camp_id)
            .fetch_all(db)
            .await?;
        Ok(reviews)
    }
}
