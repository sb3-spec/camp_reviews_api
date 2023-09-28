use std::sync::Arc;

use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use warp::{reply::Json, Filter};

use crate::auth::UserCtx;
use crate::models::{ReviewPatch, User, UserManager};

use super::models::ReviewManager;

use super::custom_warp_filters::{do_auth, with_db};

pub fn user_rest_filters(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let users_path = warp::path("users");

    let common = with_db(db.clone()).and(do_auth(db));

    let new_user_path = users_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json::<User>())
        .and(warp::path::end())
        .and_then(create_user);

    let get_user_path = users_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::end())
        .and_then(get_user);

    let delete_user_path = users_path
        .and(warp::delete())
        .and(common.clone())
        .and_then(delete_user);

    let get_user_reviews_path = users_path
        .and(warp::get())
        .and(common.clone())
        .and_then(get_user_reviews);

    new_user_path
        .or(get_user_path)
        .or(delete_user_path)
        .or(get_user_reviews_path)
}

async fn create_user(db: Arc<PgPool>, utx: UserCtx, data: User) -> Result<Json, warp::Rejection> {
    let new_user = UserManager::new_user(&db, data, utx).await?;

    json_response(new_user)
}

async fn get_user(db: Arc<PgPool>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let user = UserManager::get_user(&db, utx).await?;

    json_response(user)
}

async fn delete_user(db: Arc<PgPool>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let deleted_user = UserManager::delete_user(&db, utx).await?;

    json_response(deleted_user)
}

async fn get_user_reviews(db: Arc<PgPool>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let reviews = UserManager::get_user_reviews(&db, utx).await?;

    json_response(reviews)
}

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!(data);
    Ok(warp::reply::json(&response))
}
