extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new()
        .file("rpc.capnp")
        .run()
        .unwrap();
}
