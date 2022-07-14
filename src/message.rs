use axum::{extract::Path, response::IntoResponse, Json};
use serde::Deserialize;

use crate::State;

pub async fn sendmsg(
    Path(channelid): Path<String>,
    Json(mc): Json<MessageCreate>,
    state: State,
) -> impl IntoResponse {
    ""
}

#[derive(Deserialize)]
pub struct MessageCreate {
    pub contents: String,
}
