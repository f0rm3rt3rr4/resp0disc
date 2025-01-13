use std::io::Read;
use actix_web::{get, HttpResponse};


#[get("/")]
pub async fn index() -> HttpResponse {
    // todo ...
    let mut f = std::fs::File::open("./templates/index.html").unwrap();
    let mut s:String = String::default();
    f.read_to_string(&mut s).unwrap();
    
    HttpResponse::Ok().body(s.clone())
}