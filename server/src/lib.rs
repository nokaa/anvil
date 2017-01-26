#![feature(insert_str)]
#![recursion_limit = "1024"]

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
pub mod rpc_capnp {
    include!(concat!(env!("OUT_DIR"), "/rpc_capnp.rs"));
}
pub use server::server;
