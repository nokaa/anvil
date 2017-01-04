#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate tokio_core;
extern crate tokio_uds;

use futures::Future;
use futures::stream::Stream;
use tokio_core::io::{copy, Io};
use tokio_core::reactor::Core;
use tokio_uds::UnixListener;

use std::path::Path;

mod error;

pub use error::*;

pub fn server<P>(path: P) -> Result<()>
    where P: AsRef<Path>
{
    let mut core = Core::new().chain_err(|| "unable to start event loop")?;
    let handle = core.handle();

    let socket = UnixListener::bind(path, &handle).chain_err(|| "unable to bind to UDS")?;

    let done = socket.incoming().for_each(move |(socket, addr)| {
        let (reader, writer) = socket.split();
        let amt = copy(reader, writer);

        let msg = amt.then(move |result| {
            match result {
                Ok(amt) => println!("wrote {} bytes to {:?}", amt, addr),
                Err(e) => println!("error on {:?}: {}", addr, e),
            }

            Ok(())
        });

        handle.spawn(msg);

        Ok(())
    });

    core.run(done).chain_err(|| "unable to run event loop")
}
