extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new().file("forge.capnp").run().unwrap();
}
