use std::sync::Arc;

use super::{
    custom_warp_filters::{do_auth, with_db},
    Error,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use warp::{reply::Json, Filter};

use crate::auth::UserCtx;

use crate::models::{Camp, CampManager, CampPatch};

pub fn camp_rest_filters(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let camps_path = warp::path("camps");

    let common = with_db(db.clone()).and(do_auth(db));

    let new_camp_path = camps_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json::<CampPatch>())
        .and(warp::path::end())
        .and_then(create_camp);

    let get_all_camps_path = camps_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::end())
        .and_then(get_all_camps);

    let get_camp_path = camps_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and_then(get_camp);

    let patch_camp_path = camps_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param::<i64>())
        .and(warp::body::json::<CampPatch>())
        .and(warp::path::end())
        .and_then(update_camp);

    let delete_camp_path = camps_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param::<i64>())
        .and_then(delete_camp);

    let get_camp_reviews_path = camps_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param::<i64>())
        .and_then(get_camp_reviews);

    new_camp_path
        .or(get_camp_path)
        .or(delete_camp_path)
        .or(get_camp_reviews_path)
        .or(patch_camp_path)
        .or(get_all_camps_path)
}

async fn create_camp(
    db: Arc<PgPool>,
    utx: UserCtx,
    data: CampPatch,
) -> Result<Json, warp::Rejection> {
    let new_camp = CampManager::add_camp(&db, data, utx).await?;

    json_response(new_camp)
}

async fn get_camp(db: Arc<PgPool>, utx: UserCtx, camp_id: i64) -> Result<Json, warp::Rejection> {
    let camp = CampManager::get_camp(&db, camp_id, utx).await?;

    json_response(camp)
}

async fn delete_camp(db: Arc<PgPool>, utx: UserCtx, camp_id: i64) -> Result<Json, warp::Rejection> {
    let deleted_camp = CampManager::delete_camp(&db, camp_id, utx).await?;

    json_response(deleted_camp)
}

async fn update_camp(
    db: Arc<PgPool>,
    utx: UserCtx,
    camp_id: i64,
    data: CampPatch,
) -> Result<Json, warp::Rejection> {
    let updated_camp = CampManager::update_camp(&db, camp_id, data, utx).await?;

    json_response(updated_camp)
}

async fn get_camp_reviews(
    db: Arc<PgPool>,
    utx: UserCtx,
    camp_id: i64,
) -> Result<Json, warp::Rejection> {
    let reviews = CampManager::get_camp_reviews(&db, camp_id, utx).await?;

    json_response(reviews)
}

async fn get_all_camps(db: Arc<PgPool>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let camps = CampManager::get_all_camps(&db, utx).await?;

    json_response(camps)
}

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!(data);
    Ok(warp::reply::json(&response))
}
