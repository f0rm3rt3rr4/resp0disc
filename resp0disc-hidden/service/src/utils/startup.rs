use actix_web::http::KeepAlive::Timeout;
use actix_web::{App, HttpServer};
use std::thread::available_parallelism;
use std::time::Duration;
use actix_web::web::Data;
use tracing::{error, info};
use crate::consts::*;
use crate::app::states::AppAuthState;
use crate::routes::login::login;
use crate::routes::index::index;
use crate::routes::logout::logout;
use crate::routes::ping::ping;
use crate::utils::config::{HttpConfig, ServerConfig};

fn set_worker_num(config: &mut ServerConfig) {
    if config.worker_num == 0 {
        let result = available_parallelism();
        if result.is_ok() {
            config.worker_num = result.unwrap().get();
        } else {
            let msg: &str = "Unable to retrieve available parallelism.";
            error!(msg);
            // Sleep for 1s to wait for non-blocking trace to flush, then panic
            std::thread::sleep(Duration::from_secs(1));
            panic!("{}", msg);
        }
    }
}

fn set_worker_max_blocking_threads(config: &mut ServerConfig) {
    if config.worker_max_blocking_threads == 0 {
        config.worker_max_blocking_threads = 512 / config.worker_num;
    }
}

fn set_keepalive(config: &mut ServerConfig) {
    if config.keepalive_ms == 0 {
        config.keepalive_ms = HTTP_KEEPALIVE;
    }
}

fn set_backlog(config: &mut ServerConfig) {
    if config.backlog == 0 {
        config.backlog = HTTP_BACKLOG;
    }
}

fn set_max_connections(config: &mut ServerConfig) {
    if config.max_connections == 0 {
        config.max_connections = HTTP_MAX_CONNECTIONS;
    }
}

fn set_client_request_timeout(config: &mut ServerConfig) {
    if config.client_request_timeout_ms == 0 {
        config.client_request_timeout_ms = HTTP_CLIENT_REQUEST_TIMEOUT;
    }
}

fn set_client_disconnect_timeout(config: &mut ServerConfig) {
    if config.client_disconnect_timeout_ms == 0 {
        config.client_disconnect_timeout_ms = HTTP_DISCONNECT_TIMEOUT;
    }
}

fn set_shutdown_timeout(config: &mut ServerConfig) {
    if config.shutdown_timeout == 0 {
        config.shutdown_timeout = HTTP_SHUTDOWN_TIMEOUT;
    }
}

fn set_config_defaults(config: &ServerConfig) -> ServerConfig {
    let mut config: ServerConfig = config.clone();

    set_worker_num(&mut config);
    set_worker_max_blocking_threads(&mut config);
    set_keepalive(&mut config);
    set_backlog(&mut config);
    set_max_connections(&mut config);
    set_client_request_timeout(&mut config);
    set_client_disconnect_timeout(&mut config);
    set_shutdown_timeout(&mut config);

    config
}

pub async fn start_server(
    server_config: ServerConfig, http_config: HttpConfig
) {
    let host: &str = &server_config.host;
    let port = server_config.port;

    info!("Starting HTTP Server on {host}:{port}...");

    let server_bind = HttpServer::new(
        move || App::new()
            .app_data(
                Data::new(AppAuthState::new(http_config.auth_salt.clone()))
            )
            .service(index)
            .service(login)
            .service(logout)
            .service(ping)
    ).bind((host, port));

    if server_bind.is_ok() {
        let config = set_config_defaults(&server_config);

        info!("Server config: {:#?}", config);

        let run_result = server_bind
            .unwrap()
            .workers(config.worker_num)
            .worker_max_blocking_threads(config.worker_max_blocking_threads)
            .keep_alive(Timeout(Duration::from_millis(config.keepalive_ms)))
            .backlog(config.backlog)
            .max_connections(config.max_connections)
            .client_request_timeout(
                Duration::from_millis(config.client_request_timeout_ms)
            )
            .client_disconnect_timeout(
                Duration::from_millis(config.client_disconnect_timeout_ms)
            )
            .server_hostname(&config.server_hostname)
            .shutdown_timeout(config.shutdown_timeout)
            .run()
            .await;

        if let Err(err) = run_result {
            error!("Failed to start server on {host}:{port}, Error: {err}");
        }
    } else {
        error!("Failed to bind the server on {host}:{port}");
    }
}
