use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::State;

pub fn app(state: State) -> Router {
    Router::new()
        .route(
            "/:channelid/send",
            post(move |path, json| async {
                let state = Arc::clone(&state);
                crate::message::sendmsg(path, json, state).await
            }),
        )
        .route(
            "/ws",
            get(move |ws| async {
                let state = Arc::clone(&state);
                crate::ws::upgrade(ws, state).await
            }),
        )
}
