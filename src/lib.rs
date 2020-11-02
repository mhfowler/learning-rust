#[macro_use]
extern crate log;

mod error;
mod demo;

use std::{env, result::Result};

use jsonrpc_core::{IoHandler, Value};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};


use crate::error::BoxError;

pub fn run() -> Result<(), BoxError> {
    info!("Starting up.");

    let mut io = IoHandler::default();

    io.add_method("test_method", move |_| {
        info!("this is a test method");
        demo::demo_fun(" a test message right now");

        let return_val = String::from("42");
        Ok(Value::String(return_val))
    });


    let http_server_address = env::var("PEACH_DEMO_PORT").unwrap_or_else(|_| "127.0.0.1:5113".to_string());

    info!("Starting JSON-RPC server on {}.", http_server_address);
    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(
            &http_server_address
                .parse()
                .expect("Invalid HTTP address and port combination"),
        )
        .expect("Unable to start RPC server");

    info!("Listening for requests.");
    server.wait();

    Ok(())
}