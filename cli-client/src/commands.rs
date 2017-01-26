use anvil_server::error::*;
use anvil_rpc::editor;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use futures::Future;
use tokio_core::reactor::Core;
use tokio_core::io::Io;
use tokio_uds::UnixStream;

use std::path::Path;

pub struct Command {
    core: Core,
    editor: editor::Client<::capnp::text::Owned>,
    rpc_system: RpcSystem<rpc_twoparty_capnp::Side>,
}

impl Command {
    pub fn new<P>(path: P) -> Result<Self>
        where P: AsRef<Path>
    {
        let core = Core::new().chain_err(|| "unable to create event loop")?;
        let handle = core.handle();

        let stream = UnixStream::connect(path, &handle).chain_err(|| "unable to connect to UDS")?;
        let (reader, writer) = stream.split();

        let rpc_network = Box::new(twoparty::VatNetwork::new(reader,
                                                             writer,
                                                             rpc_twoparty_capnp::Side::Client,
                                                             Default::default()));

        let mut rpc_system = RpcSystem::new(rpc_network, None);
        let editor: editor::Client<::capnp::text::Owned> =
            rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        Ok(Command {
            core: core,
            editor: editor,
            rpc_system: rpc_system,
        })
    }

    pub fn insert(mut self, line: u64, column: u64, text: &str) -> Result<()> {
        let mut request = self.editor.insert_request();
        request.get().set_line(line);
        request.get().set_column(column);
        request.get().set_string(text);

        self.core
            .run(self.rpc_system.join(request.send().promise))
            .chain_err(|| "unable to run event loop")?;
        Ok(())
    }
}
