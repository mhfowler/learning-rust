extern crate peach_demo;
#[macro_use]
extern crate log;

use std::process;

fn main() {
    // initialize the logger
    env_logger::init();
    info!("++ starting the logger");

    // handle errors returned from `run`
    if let Err(e) = peach_demo::run() {
        error!("Application error: {}", e);
        process::exit(1);
    }
}

