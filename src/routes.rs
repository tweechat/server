use axum::{
    routing::{get, post},
    Router,
};

use crate::State;

pub fn app(state: State) -> Router {
    Router::new()
        .route(
            "/:channelid/send",
            post(move |path, json| async { crate::message::sendmsg(path, json, state).await }),
        )
        .route(
            "/ws",
            get(move |ws| async { crate::ws::upgrade(ws, state) }),
        )
}
