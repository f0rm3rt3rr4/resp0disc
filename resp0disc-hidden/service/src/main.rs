use resp0disc::utils::config::{load_config};
use resp0disc::utils::tracing::init_tracing;
use resp0disc::utils::startup::start_server;

#[actix_web::main]
async fn main() {
    let config = load_config();
    let _tracing_guards = init_tracing(&config.tracing);
    start_server(&config.server).await;
}
