use actix_web::{post, HttpRequest, HttpResponse};
use actix_web::web::{Data, Json};
use actix_web::cookie::Cookie;
use actix_web::http::StatusCode;
use serde::Deserialize;
use tracing::{info};
use crate::app::states::AppAuthState;
use crate::utils::session::{check_session, mk_session};

#[derive(Debug, Deserialize)]
struct LoginData {
    pub username: String,
    pub password: String,
}

async fn _mk_session(data: &Data<AppAuthState>) -> HttpResponse {
    let (cv, session) = mk_session(&data).await;

    info!("Session Created: {}", &session.id());

    let cookie = Cookie::build("session", &cv)
        .finish();

    HttpResponse::Ok().cookie(cookie).finish()
}

#[post("/login")]
pub async fn login(r:HttpRequest, login_data:Json<LoginData>, data: Data<AppAuthState>) -> HttpResponse {
    let _salt = &data.salt;
    info!("Login Attempt: {}:{}", login_data.username, login_data.password);

    if check_session(&r, &data).await.is_some() {
        HttpResponse::Ok().status(StatusCode::ACCEPTED).finish()
    } else {
        _mk_session(&data).await
    }
}