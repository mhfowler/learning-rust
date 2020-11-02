use std::{error};

use jsonrpc_core::{types::error::Error, ErrorCode};
use snafu::Snafu;

pub type BoxError = Box<dyn error::Error>;


#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum DemoError {
    #[snafu(display("Demo test error A: {}", msg))]
    DemoTestErrorA { msg: String },

    #[snafu(display("Demo test error B: {}", msg))]
    DemoTestErrorB { msg: String },
}

impl From<DemoError> for Error {
    fn from(err: DemoError) -> Self {
        match &err {
            DemoError::DemoTestErrorA { msg } => Error {
                code: ErrorCode::ServerError(-32001),
                message: format!("Demo test error A: {}", msg),
                data: None,
            },
            DemoError::DemoTestErrorB { msg } => Error {
                code: ErrorCode::ServerError(-32001),
                message: format!("Demo test error B: {}", msg),
                data: None,
            },
        }
    }
}
