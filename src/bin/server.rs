use env_logger::Env;
use log::{debug, error, info, log_enabled, Level};
use rsw_app; // - to _
fn main() {
    init_logger();
    rsw_app::begin_simulation();
}

fn init_logger() {
    // set $RUST_LOG env variable accordingly https://docs.rs/env_logger/0.8.2/env_logger/
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();
}
