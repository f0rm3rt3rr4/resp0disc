use actix_web::{get, HttpRequest, HttpResponse};
use actix_web::web::{Data};
use crate::app::states::AppAuthState;
use crate::utils::session::check_session;

#[get("/ping")]
pub async fn ping(r:HttpRequest, data: Data<AppAuthState>) -> HttpResponse {
    if check_session(&r, &data).await.is_some() {
        HttpResponse::Ok().body("SIGNED IN")
    } else {
        HttpResponse::Ok().body("NOT SIGNED IN")
    }
}