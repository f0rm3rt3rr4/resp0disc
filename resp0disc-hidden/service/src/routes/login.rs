use crate::app::states::AppAuthState;
use crate::utils::session::{check_session, mk_session};
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
    data: Data<Arc<AppAuthState>>,
) -> HttpResponse {
    let _salt = &data.salt;
    info!(
        "Login Attempt: {}:{}",
        login_data.username, login_data.password
    );

    let client = &data.pg_client;
    let row = client
        .query_one("SELECT * FROM users WHERE id = $1", &[&1])
        .await;

    if row.is_ok() {
        let row = row.unwrap();
        let user_name = row.get::<_, String>("user_name");
        let password = row.get::<_, String>("password");
        
        println!("value: {}:{}", user_name, password);
    } else {
        println!("Error.");
    }

    if check_session(&r, &data).await.is_some() {
        HttpResponse::Ok().status(StatusCode::ACCEPTED).finish()
    } else {
        _mk_session(&data).await
    }
}
