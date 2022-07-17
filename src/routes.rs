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
            post({
                let state = Arc::clone(&state);
                move |path, json| crate::message::sendmsg(path, json, Arc::clone(&state))
            }),
        )
        .route(
            "/ws",
            get({
                let state = Arc::clone(&state);
                move |ws| crate::ws::upgrade(ws, Arc::clone(&state))
            }),
        )
}
