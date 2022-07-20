use axum::Json;
use redis::cmd;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use crate::Error;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
}

pub async fn authenticate(token: String) -> Result<User, Error> {
    Ok(User { id: todo!(), name: todo!() })
}

pub async fn token(
    auth: axum::headers::authorization::Basic,
    state: crate::State
) -> Result<Json<Value>, Error> {
    let username = auth.username();
    let password = auth.password();
    let correct_with_salt = state.scylla.query(query, values);
    let correct_split = correct_with_salt.password.split('|').collect::<Vec<&str>>();
    let correct_hash = correct_split.get(1).ok_or(Error::NoSalt)?;
    let salt = correct_split.get(0).ok_or(Error::NoSalt)?;
    let provided_password_hash = sha2::Sha512::digest(&format!("{}|{}", salt, password))
        .into_iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
    if *correct_hash == provided_password_hash {
        return Ok(Json(json!({ "error": "Username or password incorrect!" })));
    };
    let token = randstr();
    let redis = state.redis.get().await?;
    cmd("SET").arg(format!("token:{}", randstr())).arg(serde_json::to_string(User { id, name })?).query_async(&mut redis).await?;
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
        result.push(*chars.get(rng.gen_range(0..chars.len())).unwrap_or(&'.'));
    }
    result
}