use redis::aio::AsyncStream;
use std::net::SocketAddr;
use std::sync::Arc;

mod message;
mod routes;
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

pub type State = Arc<Connections>;

pub struct Connections {
    pub redis: redis::aio::Connection<std::pin::Pin<Box<dyn AsyncStream + Send + Sync>>>,
    pub scylla: scylla::Session,
}

async fn get_state() -> Connections {
    let redis_location = std::env::var("REDIS").expect("REDIS environment variable expected!");
    let scylla_var = std::env::var("SCYLLA").expect("SCYLLA environment variable expected!");
    let scylla_locations = scylla_var.split(' ').collect::<Vec<&str>>();

    Connections {
        redis: redis::Client::open(format!("redis://{}", redis_location))
            .unwrap()
            .get_async_connection()
            .await
            .unwrap(),
        scylla: scylla::SessionBuilder::new()
            .known_nodes(&scylla_locations)
            .build()
            .await
            .unwrap(),
    }
}
