use axum::{
    extract::Path,
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    Json, TypedHeader,
};
use deadpool_redis::redis::cmd;
use serde::{Deserialize, Serialize};
use tweechat_datatypes::User;

use crate::{auth::authenticate, Error, State};

pub async fn sendmsg(
    Path(channelid): Path<String>,
    Json(mc): Json<tweechat_datatypes::MessageCreate>,
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
    Ok("")
}

#[derive(Deserialize, Serialize)]
pub struct InternalMessageCreate {
    pub contents: String,
    pub author: User,
}
