use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Redis Pool Error: {}", self)]
    RedisPool(#[from] deadpool_redis::PoolError),
    #[error("Redis Error: {}", self)]
    Redis(#[from] redis::RedisError),
    #[error("ScyllaDB Transport Error: {}", self)]
    ScyllaTrans(#[from] scylla::transport::errors::DbError),
    #[error("ScyllaDB Query Error: {}", self)]
    Scylla(#[from] scylla::transport::errors::QueryError),
    #[error("ScyllaDB From Row Error: {}", self)]
    ScyllaFromRow(#[from] scylla::cql_to_rust::FromRowError),
    #[error("Serde_Json Error: {}", self)]
    Json(#[from] serde_json::Error),
    #[error("Invalid Credentials!")]
    InvalidCredentials,
    #[error("Invalid Server-Side System Time!")]
    InvalidTime,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
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
