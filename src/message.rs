use axum::{
    extract::Path,
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    Json, TypedHeader,
};
use deadpool_redis::redis::cmd;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    auth::{authenticate, User},
    Error, State,
};

pub async fn sendmsg(
    Path(channelid): Path<String>,
    Json(mc): Json<IncomingMessageCreate>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    state: State,
) -> Result<impl IntoResponse, Error> {
    let imc = InternalMessageCreate {
        contents: mc.contents,
        author: authenticate(auth, &state).await?,
    };
    let mut conn = state.redis.get().await?;
    cmd("PUBLISH")
        .arg(&[format!("sends:{}", channelid), serde_json::to_string(&imc)?])
        .query_async(&mut conn)
        .await?;
    Ok(Json(json!({ "message": "message sent!"})))
}

#[derive(Deserialize, Serialize)]
pub struct IncomingMessageCreate {
    pub contents: String,
}

#[derive(Deserialize, Serialize)]
pub struct InternalMessageCreate {
    pub contents: String,
    pub author: User,
}
