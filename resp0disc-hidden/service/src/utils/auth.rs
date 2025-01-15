use crate::app::states::AppAuthState;
use crate::utils::sql::SQL_FETCH_USER;
use actix_web::web::Data;
use async_session::sha2;
use async_session::sha2::Digest;
use std::cmp::Ordering;
use std::sync::Arc;
use tracing::{error};

#[derive(Debug, PartialEq)]
pub enum AuthResult {
    Ok,
    NoUserFound,
    InvalidPassword,
}

pub fn hash_password(password: &str, salt: &[u8]) -> String {
    let password_with_salt = [&salt, password.as_bytes()].concat();
    hex::encode(sha2::Sha256::digest(&password_with_salt))
}

fn verify_password(
    login_password: &str, db_hash: &String, salt: &[u8]
) -> bool {
    let hash = hash_password(login_password, salt);
    hash.cmp(db_hash) == Ordering::Equal
}

pub async fn check_login_valid(
    username: &str,
    password: &str,
    auth_state: &Data<Arc<AppAuthState>>,
) -> AuthResult {
    let client = &auth_state.pg_client;
    let row = client.query_opt(SQL_FETCH_USER, &[&username]).await;

    match row {
        Ok(Some(row)) => {
            let db_password = row.get::<_, String>("password");
            let b = verify_password(&password, &db_password, &auth_state.salt);
            if b {
                AuthResult::Ok
            } else {
                AuthResult::InvalidPassword
            }
        }
        Ok(None) => AuthResult::NoUserFound,
        Err(err) => {
            error!("Unable to fetch user from the DB, error: {}", err);
            AuthResult::NoUserFound
        }
    }
}
