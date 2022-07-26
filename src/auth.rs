use crate::Error;
use axum::headers::authorization::{Basic, Bearer};
use axum::headers::Authorization;
use axum::{Json, TypedHeader};
use redis::cmd;
use scylla::IntoTypedRows;
use sha2::Digest;
use tweechat_datatypes::{LoginResponse, TotpRequest, User};

pub async fn authenticate(
    auth: Authorization<Bearer>,
    state: &crate::State,
) -> Result<User, Error> {
    let json: String = cmd("GET")
        .arg(format!("token:{}", auth.token()))
        .query_async(&mut state.redis().await?)
        .await?;
    Ok(serde_json::from_str(&json)?)
}

pub async fn token(
    TypedHeader(auth): TypedHeader<Authorization<Basic>>,
    state: crate::State,
) -> Result<Json<LoginResponse>, Error> {
    let email = auth.username();
    let password = auth.password();
    Ok(Json(gen_token(email, password, state).await?))
}

#[allow(clippy::unused_async)]
pub async fn totp(
    TypedHeader(_auth): TypedHeader<Authorization<Bearer>>,
    Json(_totp): Json<TotpRequest>,
    _state: crate::State,
) -> Result<Json<LoginResponse>, Error> {
    Ok(Json(LoginResponse {
        token: "unimplemented".to_string(),
        needs_totp: false,
    }))
}

pub async fn gen_token(
    email: &str,
    password: &str,
    state: crate::State,
) -> Result<LoginResponse, Error> {
    let (correct_email, username, correct_password_hash, salt, totp) = state
        .scylla
        .query(
            "SELECT email, username, password, salt, totp FROM twee.users WHERE email = ? ALLOW FILTERING",
            (email,),
        )
        .await?
        .rows
        .ok_or(Error::InvalidCredentials)?
        .into_typed::<(String, String, String, String, String)>()
        .next()
        .ok_or(Error::InvalidCredentials)??;
    let provided_password_hash = sha2::Sha512::digest(&format!("{}|{}", salt, password))
        .into_iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
    if correct_password_hash != provided_password_hash || correct_email != email {
        return Err(Error::InvalidCredentials);
    };
    let token = randstr(64);
    let needs_totp;
    let redis_token = if totp.is_empty() {
        needs_totp = false;
        format!("token:{}", token)
    } else {
        needs_totp = true;
        format!("token:totp:{}", token)
    };
    let mut redis = state.redis.get().await?;
    cmd("SET")
        .arg(redis_token)
        .arg(serde_json::to_string(&User {
            name: username.to_string(),
        })?)
        .query_async(&mut redis)
        .await?;
    Ok(LoginResponse { token, needs_totp })
}

#[must_use]
pub fn randstr(length: usize) -> String {
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .chars()
        .collect();
    let mut result = String::with_capacity(length);
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        result.push(
            *chars
                .get(rand::Rng::gen_range(&mut rng, 0..chars.len()))
                .unwrap_or(&'.'),
        );
    }
    result
}
