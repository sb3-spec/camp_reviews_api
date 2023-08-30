use std::{convert::Infallible, sync::Arc};

use crate::auth::{utx_from_token, UserCtx};
use sqlx::PgPool;
use warp::{Filter, Rejection};

const AUTH_HEADER: &str = "Supabase-Auth-Token";

pub fn with_db(
    db: Arc<PgPool>,
) -> impl Filter<Extract = (Arc<PgPool>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn do_auth(db: Arc<PgPool>) -> impl Filter<Extract = (UserCtx,), Error = Rejection> + Clone {
    warp::any()
        .and(with_db(db))
        .and(warp::header::optional::<String>(AUTH_HEADER))
        .and_then(|db: Arc<PgPool>, supa_auth: Option<String>| async move {
            match supa_auth {
                Some(supa_auth) => {
                    let utx = utx_from_token(&db, &supa_auth).await?;
                    Ok::<UserCtx, Rejection>(utx)
                }
                None => Err(super::Error::FailAuthMissingXAuth.into()),
            }
        })
}
