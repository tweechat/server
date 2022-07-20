#[warn(clippy::all, clippy::nursery, clippy::pedantic)]
use redis_subscribe::RedisSub;
use std::net::SocketAddr;
use std::sync::Arc;

mod auth;
mod errors;
mod message;
mod routes;
mod migrations;
mod ws;

#[tokio::main]
pub async fn main() {
    let addr: SocketAddr = ([0, 0, 0, 0], 8080).into();
    let app = routes::app(Arc::new(get_state().await));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub use errors::Error;

pub type State = Arc<Connections>;

pub struct Connections {
    pub redis: deadpool_redis::Pool,
    pub subscriber: Arc<RedisSub>,
    pub scylla: scylla::Session,
}

async fn get_state() -> Connections {
    let redis_location = std::env::var("REDIS").expect("REDIS environment variable expected!");
    let scylla_var = std::env::var("SCYLLA").expect("SCYLLA environment variable expected!");
    let scylla_locations = scylla_var.split(' ').collect::<Vec<&str>>();
    let scylla = scylla::SessionBuilder::new()
    .known_nodes(&scylla_locations)
    .build()
    .await
    .unwrap();
    migrations::migrate(&scylla).await.unwrap();
    Connections {
        redis: deadpool_redis::Config::from_url("redis://127.0.0.1/")
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .unwrap(),

        subscriber: Arc::new(RedisSub::new(&redis_location)),
        scylla,
    }
}
