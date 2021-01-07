use log::{debug, error, info, log_enabled, Level};
use std::thread;
use std::{sync::mpsc, time::Duration};

// set $RUST_LOG env variable accordingly https://docs.rs/env_logger/0.8.2/env_logger/
pub fn run() {
    init();

    let (tx, rx) = mpsc::channel();

    let tx_handle = thread::spawn(move || {
        for _ in 1..=5 {
            let val = String::from("hi");

            tx.send(val).unwrap();
        }
    });

    for received in rx {
        info!("Got: {}", received);
        thread::sleep(Duration::from_secs(2));
    }

    tx_handle.join().expect("transmitter panicked");

    info!("..Done..");
}

fn init() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();
}
