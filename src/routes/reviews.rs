use std::sync::Arc;

use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use warp::{reply::Json, Filter};

use crate::auth::UserCtx;
use crate::models::ReviewPatch;

use super::models::ReviewManager;

use super::custom_warp_filters::{do_auth, with_db};

pub fn review_rest_filters(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let reviews_path = warp::path("reviews");

    let common = with_db(db.clone()).and(do_auth(db));

    let get_camp_reviews_route = reviews_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and_then(get_camp_reviews);

    let create_review_route = reviews_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json::<ReviewPatch>())
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and_then(create_review);

    let delete_review_route = reviews_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and_then(delete_review);

    let delete_camp_reviews_route = reviews_path
        .and(warp::path("camp_id"))
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and_then(delete_all_camp_reviews);

    get_camp_reviews_route
        .or(create_review_route)
        .or(delete_review_route)
        .or(delete_camp_reviews_route)
}

async fn get_camp_reviews(
    db: Arc<PgPool>,
    _utx: UserCtx,
    camp_id: i64,
) -> Result<Json, warp::Rejection> {
    let reviews = ReviewManager::get_camp_reviews(&db, camp_id).await?;

    json_response(reviews)
}

async fn create_review(
    db: Arc<PgPool>,
    utx: UserCtx,
    data: ReviewPatch,
    camp_id: i64,
) -> Result<Json, warp::Rejection> {
    let review = ReviewManager::create(&db, utx, data, camp_id).await?;

    json_response(review)
}

async fn delete_review(
    db: Arc<PgPool>,
    utx: UserCtx,
    review_id: i64,
) -> Result<Json, warp::Rejection> {
    ReviewManager::delete(&db, &utx, review_id).await?;

    json_response(())
}

async fn delete_all_camp_reviews(
    db: Arc<PgPool>,
    utx: UserCtx,
    camp_id: i64,
) -> Result<Json, warp::Rejection> {
    ReviewManager::delete_all_camp_reviews(&db, utx, camp_id).await?;

    json_response(())
}

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!(data);
    Ok(warp::reply::json(&response))
}
