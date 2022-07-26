#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use deadpool_redis::Connection;
use redis_subscribe::RedisSub;
use std::net::SocketAddr;
use std::sync::Arc;

mod account;
mod auth;
mod errors;
mod message;
mod migrations;
mod routes;
mod ws;

/// Main
/// # Panics
/// Panics if the address is unavaliable
#[tokio::main]
pub async fn main() {
    let addr: SocketAddr = ([0, 0, 0, 0], 8080).into();
    let app = routes::app(&Arc::new(get_state().await));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub use errors::Error;

pub type State = Arc<Connections>;

pub struct Connections {
    redis: deadpool_redis::Pool,
    pub subscriber: Arc<RedisSub>,
    pub scylla: scylla::Session,
}

impl Connections {
    /// A sugar for state.redis.get().await
    async fn redis(&self) -> Result<Connection, deadpool_redis::PoolError> {
        self.redis.get().await
    }
}

async fn get_state() -> Connections {
    let redis_location = format!(
        "redis://{}/",
        std::env::var("REDIS").expect("REDIS environment variable expected!")
    );
    let scylla_var = std::env::var("SCYLLA").expect("SCYLLA environment variable expected!");
    let scylla_locations = scylla_var.split(' ').collect::<Vec<&str>>();
    let scylla = scylla::SessionBuilder::new()
        .known_nodes(&scylla_locations)
        .build()
        .await
        .unwrap();
    migrations::migrate(&scylla).await.unwrap();
    let redis = deadpool_redis::Config::from_url(&redis_location)
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .unwrap();
    println!("{:?}", redis.status());
    Connections {
        redis,
        subscriber: Arc::new(RedisSub::new(&redis_location)),
        scylla,
    }
}
