use axum::{response::IntoResponse, Json};
use sha2::Digest;
use tweechat_datatypes::UserCreate;

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
