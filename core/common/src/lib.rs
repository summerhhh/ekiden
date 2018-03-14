#![feature(use_extern_macros)]

extern crate ekiden_common;
extern crate ekiden_enclave_common;
extern crate ekiden_rpc_common;

pub use ekiden_common::*;
pub use ekiden_common::error::{self, Error, Result};
pub use ekiden_enclave_common::quote;

pub use ekiden_rpc_common::rpc_api;

pub mod rpc {
    pub use ekiden_rpc_common::*;
}
