use crate::model::Db;
use std::{convert::Infallible, sync::Arc};
use warp::Filter;

pub fn todo_rest_filters(
    base_path: &'static str,
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
}

// region: Filter Utils
pub fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn do_auth(db: Arc<Db>) -> impl Filter<Extract = (UserCtx,), Error = Rejection> + Clone {
    warp::any().and_then(|| async { Ok::<UseCtx, Rejection>(utx_from_token("123").await.unwrap()) })
}
// endregion: Filter Utils
