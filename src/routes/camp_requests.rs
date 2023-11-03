use std::sync::Arc;

use super::{
    custom_warp_filters::{do_auth, with_db},
    json_response,
};

use sqlx::PgPool;
use warp::{reply::Json, Filter};

use crate::{auth::UserCtx, models::camp_request::CampRequestManager};

use crate::models::{Camp, CampManager, CampPatch};

pub fn camp_requests_rest_filters(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let common = with_db(db.clone()).and(do_auth(db));
    let camp_requests_path = warp::path("camp_requests");

    let new_camp_request_path = camp_requests_path
        .and(common.clone())
        .and(warp::post())
        .and(warp::body::json::<CampPatch>())
        .and_then(new_camp_request);

    let get_camp_requests_path = camp_requests_path
        .and(common.clone())
        .and(warp::get())
        .and(warp::path::end())
        .and_then(get_camp_requests);

    let delete_camp_request_path = camp_requests_path
        .and(common.clone())
        .and(warp::delete())
        .and(warp::path::param::<i64>())
        .and_then(delete_camp_request);

    new_camp_request_path
        .or(get_camp_requests_path)
        .or(delete_camp_request_path)
}

pub async fn new_camp_request(
    db: Arc<PgPool>,
    utx: UserCtx,
    data: CampPatch,
) -> Result<Json, warp::Rejection> {
    let new_camp = CampRequestManager::new_request(&db, &utx, data).await?;

    json_response(new_camp)
}

pub async fn get_camp_requests(db: Arc<PgPool>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let camp_requests = CampRequestManager::get_camp_requests(&db, utx).await?;

    json_response(camp_requests)
}

pub async fn delete_camp_request(
    db: Arc<PgPool>,
    utx: UserCtx,
    camp_request_id: i64,
) -> Result<Json, warp::Rejection> {
    CampRequestManager::delete_camp_request(&db, &utx, camp_request_id).await?;

    json_response("Ok".to_string())
}
