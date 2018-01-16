#![cfg_attr(feature = "sgx", no_std)]
#![cfg_attr(feature = "sgx", feature(prelude_import))]

extern crate protobuf;
extern crate sodalite;
extern crate byteorder;

#[cfg(feature = "sgx")]
#[cfg_attr(feature = "sgx", macro_use)]
extern crate sgx_tstd as std;

#[cfg(feature = "sgx")]
extern crate sgx_trts;

#[cfg(not(feature = "sgx"))]
extern crate rand;

#[cfg_attr(feature = "sgx", allow(unused))]
#[cfg_attr(feature = "sgx", prelude_import)]
#[cfg(feature = "sgx")]
use std::prelude::v1::*;

pub mod address;
pub mod contract;
pub mod contract_error;
pub mod secure_channel;

mod generated;

#[macro_use]
mod macros;

pub use address::Address;
pub use contract::*;
pub use contract_error::ContractError;

pub mod api {
    pub use generated::enclave_rpc::*;
}