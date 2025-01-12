use actix_web::{App, HttpServer};
use tracing::{error, info};
use resp0disc::config::{load_config, ServerConfig};
use resp0disc::tracing::init_tracing;

async fn start_server(config: &ServerConfig)  {
    let host: &str = &config.host;
    let port = config.port;

    info!("Starting HTTP Server on {host}:{port} with config: {:#?}", config);

    let server_bind = HttpServer::new(|| App::new()).bind((host, port));

    if server_bind.is_ok() {
        let run_result = server_bind
            .unwrap()
            .run()
            .await;
        if let Err(err) = run_result {
            error!("Failed to start server on {host}:{port}, Error: {err}");
        }
    } else {
        error!("Failed to bind the server on {host}:{port}");
    }
}

#[actix_web::main]
async fn main() {
    let config = load_config();
    let _tracing_guards = init_tracing(&config.tracing);
    start_server(&config.server).await;
}
