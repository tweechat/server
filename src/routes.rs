use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::State;

pub fn app(state: &State) -> Router {
    let auth = Router::new()
        .route(
            "/token",
            get({
                let state = Arc::clone(state);
                move |auth| crate::auth::token(auth, Arc::clone(&state))
            }),
        )
        .route(
            "/totp",
            post({
                let state = Arc::clone(state);
                move |auth, json| crate::auth::totp(auth, json, Arc::clone(&state))
            }),
        );
    let account = Router::new().route(
        "/create",
        post({
            let state = Arc::clone(state);
            move |json| crate::account::create(json, Arc::clone(&state))
        }),
    );
    Router::new()
        .nest("/auth", auth)
        .nest("/account", account)
        .route(
            "/:username/send",
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
