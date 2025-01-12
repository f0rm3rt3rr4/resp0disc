use tracing::{info, Subscriber};
use tracing_appender::{non_blocking, rolling};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::{Layer};
use tracing_subscriber::fmt::writer::{BoxMakeWriter, WithMaxLevel};
use tracing_subscriber::fmt::writer::{MakeWriterExt};
use tracing_subscriber::layer::{Layered, SubscriberExt};
use tracing_subscriber::Registry;
use tracing_subscriber::registry::LookupSpan;
use crate::utils::config::TracingConfig;
use crate::consts;


fn mk_layer<S>(
    config: &TracingConfig,
    writer: WithMaxLevel<NonBlocking>,
    registry: S,
    with_ansi: bool,
) -> Layered<Box<dyn Layer<S> + Send + Sync>, S, S>
where S: Subscriber + for<'a> LookupSpan<'a> + Send + Sync,
{
     let layer = tracing_subscriber::fmt::Layer::default()
        .with_writer(BoxMakeWriter::new(writer))
        .with_file(config.file_name)
        .with_line_number(config.line_number)
        .with_thread_ids(config.thread_id)
        .with_target(config.event_target)
        .with_level(true)
        .with_ansi(with_ansi)
        .compact()
        .boxed();

    registry.with(layer)
}

pub fn init_tracing(config: &TracingConfig) -> Vec<WorkerGuard> {
    let mut guards: Vec<WorkerGuard> = vec![];

    let registry = Registry::default();

    let (stdout, _guard) = non_blocking(std::io::stdout());
    let stdout = stdout.with_max_level(config.max_level.to_level());
    guards.push(_guard);

    let registry = mk_layer(config, stdout, registry, true);

    let registry = if config.log_to_file {
        let file_appender = rolling::hourly(&config.log_dir, consts::LOG_FILE_NAME);
        let (file, _guard) = tracing_appender::non_blocking(file_appender);
        let file = file.with_max_level(config.max_level.to_level());
        guards.push(_guard);

        mk_layer(&config, file, registry, false)
    } else {
        // We add stdout again, which does nothing - so we fulfill the type
        // expectation
        let (stdout, _guard) = non_blocking(std::io::stdout());
        let stdout = stdout.with_max_level(config.max_level.to_level());
        guards.push(_guard);

        mk_layer(&config, stdout, registry, false)
    };

    tracing::subscriber::set_global_default(registry)
        .expect("Unable to set tracing default global subscriber");

    info!("Tracing initialized with config: {:#?}", config);

    guards
}
