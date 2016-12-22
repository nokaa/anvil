pub use forge_rpc::*;

mod forge_rpc {
    include!(concat!(env!("OUT_DIR"), "/forge_capnp.rs"));
}
