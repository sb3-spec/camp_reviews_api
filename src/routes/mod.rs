use std::{convert::Infallible, sync::Arc};

use serde_json::json;
use sqlx::PgPool;
use warp::{reject::Rejection, Filter};

use crate::{auth, models};

use self::reviews::review_rest_filters;

mod custom_warp_filters;
mod reviews;

pub async fn start_web(web_port: u16, db: Arc<PgPool>) -> Result<(), Error> {
    let cors = warp::cors()
        .allow_origins(["http://localhost:5173"])
        .allow_headers(vec!["Supabase-Auth-Token", "Content-Type", "content-type"])
        .allow_methods(vec!["GET", "POST", "HEAD", "DELETE", "PATCH"]);

    let apis = review_rest_filters(db.clone());

    let content = warp::fs::dir("web-folder/".to_string());

    let root_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("web_folder/index.html"));

    let static_site = root_index.or(content);

    // Combine the routes!
    let routes = static_site.or(apis).with(cors).recover(handle_rejection);
    println!("Starting web server at 0.0.0.0:{}", web_port);
    warp::serve(routes).run(([0, 0, 0, 0], web_port)).await;

    Ok(())
}

async fn handle_rejection(err: Rejection) -> Result<impl warp::Reply, Infallible> {
    let mut error_code = warp::http::StatusCode::BAD_REQUEST;
    let mut error_message = String::new();

    match err.find::<WebErrorMessage>() {
        Some(e) => error_message = e.message.to_owned(),
        None => (),
    };

    let result = json!({ "error": error_message });
    let result = warp::reply::json(&result);

    println!("{:?}", err);

    Ok(warp::reply::with_status(result, error_code))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' not found.")]
    FailStartWebFolderNotFound(String),

    #[error{"Fail authentication missing X-Auth-Token header."}]
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
