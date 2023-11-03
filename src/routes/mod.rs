use std::{convert::Infallible, sync::Arc};

use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use warp::{reject::Rejection, reply::Json, Filter};

use crate::{
    auth, models,
    routes::{
        camp_requests::camp_requests_rest_filters, camps::camp_rest_filters,
        users::user_rest_filters,
    },
};

use self::reviews::review_rest_filters;

mod camp_requests;
mod camps;
mod custom_warp_filters;
mod reviews;
mod users;

pub async fn start_web(web_port: u16, db: Arc<PgPool>) -> Result<(), Error> {
    let cors = warp::cors()
        .allow_origins(["http://localhost:5173"])
        .allow_headers(vec!["Supabase-Auth-Token", "Content-Type", "content-type"])
        .allow_methods(vec!["GET", "POST", "HEAD", "DELETE", "PATCH", "OPTIONS"]);

    let api = review_rest_filters(db.clone())
        .or(user_rest_filters(db.clone()))
        .or(camp_rest_filters(db.clone()))
        .or(camp_requests_rest_filters(db.clone()));

    let content = warp::fs::dir("web-folder/".to_string());

    let root_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("web_folder/index.html"));

    let static_site = root_index.or(content);

    // Combine the routes!
    let routes = static_site.or(api).with(cors).recover(handle_rejection);
    println!("Starting web server at 0.0.0.0:{}", web_port);
    warp::serve(routes).run(([0, 0, 0, 0], web_port)).await;

    Ok(())
}

async fn handle_rejection(err: Rejection) -> Result<impl warp::Reply, Infallible> {
    let mut _error_code = warp::http::StatusCode::BAD_REQUEST;
    let mut error_message = String::new();

    match err.find::<WebErrorMessage>() {
        Some(e) => error_message = e.message.to_owned(),
        None => (),
    };

    let result = json!({ "error": error_message });
    let result = warp::reply::json(&result);

    Ok(warp::reply::with_status(result, _error_code))
}

pub fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!(data);
    Ok(warp::reply::json(&response))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' not found.")]
    FailStartWebFolderNotFound(String),

    #[error{"Fail authentication missing Supabase-Auth-Token header."}]
    FailAuthMissingXAuth,
}

// region: Warp Custom Error
#[derive(Debug)]
pub struct WebErrorMessage {
    pub typ: &'static str,
    pub message: String,
}

impl warp::reject::Reject for WebErrorMessage {}

impl WebErrorMessage {
    pub fn rejection(typ: &'static str, message: String) -> warp::Rejection {
        warp::reject::custom(WebErrorMessage { typ, message })
    }
}

impl From<self::Error> for warp::Rejection {
    fn from(other: self::Error) -> Self {
        WebErrorMessage::rejection("web::Error", format!("{:?}", other))
    }
}

impl From<models::Error> for warp::Rejection {
    fn from(other: models::Error) -> Self {
        WebErrorMessage::rejection("web::Error", format!("{:?}", other))
    }
}

impl From<auth::Error> for warp::Rejection {
    fn from(other: auth::Error) -> Self {
        WebErrorMessage::rejection("web::Error", format!("{:?}", other))
    }
}

// endregion: Warp Custom Error
