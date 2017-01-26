extern crate anvil_server;

fn main() {
    anvil_server::server("\0anvil_uds").unwrap();
}
