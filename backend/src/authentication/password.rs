use anyhow::Context;
use argon2::password_hash::{SaltString, Error};
use argon2::{Argon2, PasswordHash};

use crate::error::ApiError;
use crate::response::ApiResult;

pub async fn verify_password_argon2(password: String, password_hash: &str) -> ApiResult<()> {
    let password_hash = password_hash.to_owned();
    tokio::task::spawn_blocking(move || -> ApiResult<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                Error::Password => ApiError::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")?
}

pub async fn hash_password_argon2(password: String) -> ApiResult<String> {
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(rand::thread_rng());
        let hash = PasswordHash::generate(Argon2::default(), password, salt.as_salt())
            .map_err(|_| anyhow::anyhow!("Failed to hash password"))?;
        Ok(hash.to_string())
    })
    .await
    .context("failed to hash password")?
}