use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sha2::Digest;

use crate::{auth::randstr, Error, State};

pub async fn create(Json(uc): Json<UserCreate>, state: State) -> Result<impl IntoResponse, Error> {
    let salt = randstr(48);
    let hashed_password = sha2::Sha512::digest(&format!("{}|{}", salt, uc.password))
        .into_iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
    state.scylla.query("INSERT INTO twee.users (username, email, password, salt, totp, privkey, pubkey) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", (uc.name, uc.email, hashed_password, salt, "", uc.privkey, uc.pubkey, uc.security_level.to_string())).await?;
    Ok("")
}

#[derive(Serialize, Deserialize)]
pub struct UserCreate {
    pub email: String,
    pub name: String,
    pub password: String,
    pub pubkey: String,
    pub privkey: String,
    pub security_level: UserSecurityLevel
}

#[derive(Serialize, Deserialize)]
pub enum UserSecurityLevel {
    Max,
    Reasonable,
    Low,
}

impl ToString for UserSecurityLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Max => "Max".to_string(),
            Self::Reasonable => "Reasonable".to_string(),
            Self::Low => "Low".to_string(),
        }
    }
}
