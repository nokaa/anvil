#![feature(insert_str)]
#![recursion_limit = "1024"]

extern crate anvil_rpc;
#[macro_use]
extern crate error_chain;
extern crate capnp;
#[macro_use]
extern crate capnp_rpc;
extern crate futures;
extern crate tokio_core;
extern crate tokio_uds;
extern crate xi_rope;

mod server;

pub mod error;
pub use server::server;
