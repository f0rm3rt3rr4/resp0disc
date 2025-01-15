use actix_web::rt::spawn;
use actix_web::rt::time::sleep;
use async_redis_session::RedisSessionStore;
use std::time::Duration;
use tokio_postgres::{connect, Client, NoTls};
use tracing::error;

pub struct AppAuthState {
    pub redis: RedisSessionStore,
    pub pg_client: Client,
    pub salt: Vec<u8>,
}

impl AppAuthState {
    pub async fn new(salt: String) -> Self {
        let redis = RedisSessionStore::new("redis://127.0.0.1:6379/0");

        //todo handle errors
        let (pg_client, pg_conn) = connect(
            "host=127.0.0.1 \
            dbname=resp0disc \
            port=5432 \
            user=postgres \
            password=postgres",
            NoTls,
        )
        .await
        .unwrap();
        spawn(pg_conn);

        if redis.is_ok() {
            let redis = redis.unwrap();
            Self {
                redis,
                pg_client,
                salt: salt.into_bytes()
            }
        } else {
            error!("Could not connect to Redis");
            // Sleep for 1s to wait for non-blocking trace to flush, then panic
            sleep(Duration::from_secs(1)).await;
            panic!("Could not connect to Redis");
        }
    }
}
