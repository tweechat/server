use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::State;

pub fn app(state: &State) -> Router {
    Router::new()
        .route(
            "/auth/new",
            get({
                let state = Arc::clone(state);
                move |json| crate::account::create(json, Arc::clone(&state))
            }),
        )
        .route(
            "/auth/token",
            get({
                let state = Arc::clone(state);
                move |auth| crate::auth::token(auth, Arc::clone(&state))
            }),
        )
        .route(
            "/:channelid/send",
            post({
                let state = Arc::clone(state);
                move |path, json, auth| {
                    crate::message::sendmsg(path, json, auth, Arc::clone(&state))
                }
            }),
        )
        .route(
            "/ws",
            get({
                let state = Arc::clone(state);
                move |ws, auth| crate::ws::upgrade(ws, auth, Arc::clone(&state))
            }),
        )
}
