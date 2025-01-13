use std::time::Duration;
use async_redis_session::RedisSessionStore;
use tracing::error;

pub struct AppAuthState {
    pub store: RedisSessionStore,
    pub salt: String,
}

impl AppAuthState {
    pub fn new(salt: String) -> Self {
        let store = RedisSessionStore::new("redis://127.0.0.1:6379/0");
        if store.is_ok() {
            let store = store.unwrap();
            Self { store, salt }
        } else {
            error!("Could not connect to Redis");
            // Sleep for 1s to wait for non-blocking trace to flush, then panic
            std::thread::sleep(Duration::from_secs(1));
            panic!("Could not connect to Redis");
        }
    }
}
