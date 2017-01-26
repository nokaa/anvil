extern crate capnp;

mod rpc_capnp {
    include!(concat!(env!("OUT_DIR"), "/rpc_capnp.rs"));
}

pub use rpc_capnp::*;
