use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Redis Pool Error: {}", self)]
    RedisPool(#[from] deadpool_redis::PoolError),
    #[error("Redis Error: {}", self)]
    Redis(#[from] redis::RedisError),
    #[error("ScyllaDB Transport Error: {}", self)]
    Scylla(#[from] scylla::transport::errors::DbError),
    #[error("Serde_Json Error: {}", self)]
    Json(#[from] serde_json::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Error::RedisPool(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Redis(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Scylla(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Json(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        axum::response::Response::builder()
            .header(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static("application/json"),
            )
            .status(status)
            .body(axum::body::boxed(axum::body::Full::from(
                serde_json::json!({ "error": format!("{:?}", self) }).to_string(),
            )))
            .unwrap()
    }
}
