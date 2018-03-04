#![feature(use_extern_macros)]
#![no_std]

extern crate ekiden_db_trusted;
extern crate ekiden_rpc_trusted;

pub mod rpc {
    pub use ekiden_rpc_trusted::*;
}

pub mod db {
    pub use ekiden_db_trusted::*;
}