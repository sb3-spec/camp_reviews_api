use std::sync::Arc;

use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use warp::{reply::Json, Filter};

use crate::auth::UserCtx;
use crate::models::{
    favorite_camps::{UserCampJunction, UserCampJunctionManager},
    User, UserManager,
};

use super::custom_warp_filters::{do_auth, with_db};

pub fn user_rest_filters(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let users_path = warp::path("users");

    let common = with_db(db.clone()).and(do_auth(db));

    // region: Paths
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
        .and(warp::path::end())
        .and_then(delete_user);

    let get_user_reviews_path = users_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::end())
        .and_then(get_user_reviews);

    let add_camp_to_favorites_path = users_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::path("favorite"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and_then(add_camp_to_favorites_handler);

    let remove_camp_from_favorites_path = users_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path("favorite"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and_then(remove_camp_from_favorites_handler);

    let get_favorite_camps_path = users_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path("favorite"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(get_favorite_camps);

    // endregion: Paths
    new_user_path
        .or(get_user_path)
        .or(delete_user_path)
        .or(get_user_reviews_path)
        .or(add_camp_to_favorites_path)
        .or(remove_camp_from_favorites_path)
        .or(get_favorite_camps_path)
}

// region: Handlers
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

async fn add_camp_to_favorites_handler(
    db: Arc<PgPool>,
    utx: UserCtx,
    camp_id: i64,
) -> Result<Json, warp::Rejection> {
    UserCampJunctionManager::favorite(&db, utx, camp_id).await?;

    json_response("Camp added to favorites")
}

async fn get_favorite_camps(
    db: Arc<PgPool>,
    _utx: UserCtx,
    user_id: String,
) -> Result<Json, warp::Rejection> {
    let camps = UserCampJunctionManager::qyery_by_user_id(&db, user_id).await?;

    json_response(camps)
}

async fn remove_camp_from_favorites_handler(
    db: Arc<PgPool>,
    utx: UserCtx,
    camp_id: i64,
) -> Result<Json, warp::Rejection> {
    UserCampJunctionManager::unfavorite(&db, utx, camp_id).await?;
    json_response("Camp removed from favorites")
}

// endregion: Handlers

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!(data);
    Ok(warp::reply::json(&response))
}
