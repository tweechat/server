use axum::{extract::Path, response::IntoResponse, Json};
use deadpool_redis::redis::cmd;
use serde::{Deserialize, Serialize};

use crate::{auth::User, Error, State};

pub async fn sendmsg(
    Path(channelid): Path<String>,
    Json(mc): Json<MessageCreate>,
    state: State,
) -> Result<impl IntoResponse, Error> {
    let mut conn = state.redis.get().await?;
    cmd("PUBLISH")
        .arg(&[format!("sends:{}", channelid), serde_json::to_string(&mc)?])
        .query_async(&mut conn)
        .await?;
    Ok("")
}

#[derive(Deserialize, Serialize)]
pub struct MessageCreate {
    pub contents: String,
    pub author: User,
}
