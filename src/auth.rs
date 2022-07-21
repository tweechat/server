use crate::Error;
use axum::headers::authorization::{Basic, Bearer};
use axum::Json;
use redis::cmd;
use scylla::IntoTypedRows;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::Digest;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
}

pub async fn authenticate(auth: Bearer, state: &crate::State) -> Result<User, Error> {
    let json: String = cmd("GET")
        .arg(format!("token:{}", auth.token()))
        .query_async(&mut state.redis().await?)
        .await?;
    Ok(serde_json::from_str(&json)?)
}

pub async fn token(auth: Basic, state: crate::State) -> Result<Json<Value>, Error> {
    let email = auth.username();
    let password = auth.password();
    let (id, correct_email, username, correct_password, salt, totp) = state
        .scylla
        .query("SELECT id, email, username, password, salt, totp FROM", &[])
        .await?
        .rows
        .ok_or(Error::InvalidCredentials)?
        .into_typed::<(i64, String, String, String, String, String)>()
        .nth(0)
        .ok_or(Error::InvalidCredentials)??;
    let provided_password_hash = sha2::Sha512::digest(&format!("{}|{}", salt, password))
        .into_iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
    let correct_password_hash = sha2::Sha512::digest(&format!("{}|{}", salt, correct_password))
        .into_iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
    if correct_password_hash != provided_password_hash || correct_email != email {
        return Err(Error::InvalidCredentials);
    };
    let token = randstr();
    let mut redis = state.redis.get().await?;
    cmd("SET")
        .arg(format!("token:{}", randstr()))
        .arg(serde_json::to_string(&User {
            id,
            name: username.to_string(),
        })?)
        .query_async(&mut redis)
        .await?;
    Ok(Json(json!({ "token": token })))
}

#[must_use]
pub fn randstr() -> String {
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .chars()
        .collect();
    let mut result = String::with_capacity(64);
    let mut rng = rand::thread_rng();
    for _ in 0..64 {
        result.push(
            *chars
                .get(rand::Rng::gen_range(&mut rng, 0..chars.len()))
                .unwrap_or(&'.'),
        );
    }
    result
}
