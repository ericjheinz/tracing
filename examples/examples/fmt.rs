#![deny(rust_2018_idioms)]
use std::io;
use tracing::{debug, Level};
use tracing_subscriber::{
    layer::Layer,
    registry::{FmtLayer, Registry},
};

#[path = "fmt/yak_shave.rs"]
mod yak_shave;

fn main() {
    let stderr = FmtLayer::builder()
        .with_interest(|event| event.metadata().level() >= &Level::WARN)
        .with_writer(io::stderr)
        .build();

    let stdout = FmtLayer::builder()
        .with_interest(|event| event.metadata().level() == &Level::INFO)
        .with_writer(io::stdout)
        .build();

    let subscriber = stderr.and_then(stdout).with_subscriber(Registry::default());
    tracing::subscriber::set_global_default(subscriber).expect("Could not set global default");

    let number_of_yaks = 3;
    debug!("preparing to shave {} yaks", number_of_yaks);

    let number_shaved = yak_shave::shave_all(number_of_yaks);

    debug!(
        message = "yak shaving completed.",
        all_yaks_shaved = number_shaved == number_of_yaks,
    );
}
