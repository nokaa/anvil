use capnp;

use std::io;

error_chain!{
    foreign_links {
        Io(io::Error);
        Capnp(capnp::Error);
    }
}
