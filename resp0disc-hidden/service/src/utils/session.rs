use crate::app::states::AppAuthState;
use actix_web::web::Data;
use actix_web::HttpRequest;
use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

fn id_from_cookie(s: String) -> String {
    s.split("=")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap_or(&"")
        .to_string()
}

pub async fn destroy_session(cookie_value: String, store: &RedisSessionStore) {
    let session = store
        .load_session(id_from_cookie(cookie_value))
        .await
        .unwrap();
    
    match session {
        Some(mut sess) => {
            sess.expire_in(Duration::from_millis(0));
            store.destroy_session(sess).await.unwrap();
        }
        None => {
            info!("Session cookie not found.");
        }
    }
}

pub async fn get_session(
    cookie_value: String, store: &RedisSessionStore
) -> Option<Session> {
    let session = store.load_session(id_from_cookie(cookie_value)).await;
    if session.is_ok() {
        let session = session.unwrap();
        match session {
            Some(sess) => Some(sess),
            None => None,
        }
    } else {
        None
    }
}

pub async fn check_session(
    r: &HttpRequest,
    data: &Data<Arc<AppAuthState>>
) -> Option<Session> {
    let cookie = r.cookie("session");
    if cookie.is_some() {
        let c = cookie.unwrap();
        let s = get_session(c.to_string(), &data.redis).await;
        if s.is_some() {
            let s = s.unwrap();
            info!("Logged in: {:?}", &s.id());
            Some(s)
        } else {
            None
        }
    } else {
        None
    }
}

pub async fn mk_session(auth_state: &Data<Arc<AppAuthState>>) -> (String, Session) {
    let store = &auth_state.redis;
    let mut session = Session::new();
    session.expire_in(Duration::from_secs(500));
    let cookie_value = store.store_session(session).await.unwrap().unwrap();
    let cv = cookie_value.clone();
    let session = store.load_session(cookie_value).await.unwrap();
    let session = session.unwrap();

    (cv, session)
}
