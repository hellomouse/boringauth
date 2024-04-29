use tracing::info;

mod common;

fn main() {
    common::initialize_logging();
    info!("Hello, world!");
}
