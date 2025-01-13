use crate::app::states::AppAuthState;
use actix_web::web::{Data,};
use actix_web::{post, HttpRequest, HttpResponse};
use actix_web::http::StatusCode;
use tracing::{info};
use crate::utils::session::{check_session, destroy_session};

#[post("/logout")]
pub async fn logout(
    r:HttpRequest,
    auth_state: Data<AppAuthState>,
) -> HttpResponse {
    let s = check_session(&r, &auth_state).await;
    if s.is_some() {
        let cookie = r.cookie("session").unwrap();
        info!("Ending Session: {}", s.unwrap().id());
        destroy_session(cookie.to_string(), &auth_state.store).await;
    } else {
        info!("No Session tp end.");
    }

    HttpResponse::Ok().status(StatusCode::OK).finish()
}
