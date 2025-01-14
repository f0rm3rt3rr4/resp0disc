use crate::app::states::AppAuthState;
use crate::utils::auth::{check_login_valid, AuthResult};
use crate::utils::session::{get_session, mk_session};
use actix_web::cookie::Cookie;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use actix_web::{post, HttpRequest, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Deserialize)]
struct LoginData {
    pub username: String,
    pub password: String,
}

async fn _mk_session(data: &Data<Arc<AppAuthState>>) -> HttpResponse {
    let (cv, session) = mk_session(&data).await;

    info!("Session Created: {}", &session.id());

    let cookie = Cookie::build("session", &cv).finish();

    HttpResponse::Ok().cookie(cookie).finish()
}

#[post("/login")]
pub async fn login(
    r: HttpRequest,
    login_data: Json<LoginData>,
    auth_state: Data<Arc<AppAuthState>>,
) -> HttpResponse {
    info!(
        "Login Attempt: {}:{}",
        login_data.username, login_data.password
    );

    if get_session(&r, &auth_state).await.is_some() {
        HttpResponse::Ok().status(StatusCode::ACCEPTED).finish()
    } else {
        match check_login_valid(
            &login_data.username,
            &login_data.password,
            &auth_state
        ).await {
            AuthResult::Ok => _mk_session(&auth_state).await,
            AuthResult::NoUserFound => {
                info!(
                    "Unable to login, no user found: {}", &login_data.username
                );
                HttpResponse::Unauthorized().finish()
            }
            AuthResult::InvalidPassword => {
                info!(
                    "Unable to login, invalid password: {}",
                    &login_data.username
                );
                HttpResponse::Unauthorized().finish()
            }
        }
    }
}
