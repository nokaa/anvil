use capnp;

use std::io;

error_chain!{
    foreign_links {
        Capnp(capnp::Error);
        Io(io::Error);
    }
}
