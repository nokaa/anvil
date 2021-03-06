use error::*;

use anvil_rpc::editor;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use futures::Future;
use tokio_core::reactor::Core;
use tokio_core::io::Io;
use tokio_uds::UnixStream;

use std::path::Path;

pub struct Command {
    core: Core,
    editor: editor::Client,
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
        let editor: editor::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        handle.spawn(rpc_system.map_err(|_e| ()));

        Ok(Command {
            core: core,
            editor: editor,
        })
    }

    pub fn open_file(mut self, file_name: &str) -> Result<()> {
        let mut request = self.editor.open_file_request();
        request.get().set_path(file_name);

        self.core
            .run(request.send().promise)
            .chain_err(|| "unable to run event loop")?;
        Ok(())
    }

    pub fn write_file(mut self, file_name: &str) -> Result<()> {
        let mut request = self.editor.write_file_request();
        request.get().set_path(file_name);

        self.core
            .run(request.send().promise)
            .chain_err(|| "unable to run event loop")?;
        Ok(())
    }

    pub fn get(mut self, start: u64, end: u64) -> Result<String> {
        let mut request = self.editor.get_request();
        request.get().set_start_line(start);
        request.get().set_end_line(end);

        let response = self.core
            .run(request.send().promise)
            .chain_err(|| "unable to run event loop")?;
        let lines = response.get()?.get_lines()?;
        Ok(lines.to_string())
    }

    pub fn insert(mut self, line: u64, column: u64, text: &str) -> Result<()> {
        let mut request = self.editor.insert_request();
        request.get().set_line(line);
        request.get().set_column(column);
        request.get().set_string(text);

        self.core
            .run(request.send().promise)
            .chain_err(|| "unable to run event loop")?;
        Ok(())
    }

    pub fn delete(mut self, line: u64, column: u64, length: u64) -> Result<()> {
        let mut request = self.editor.delete_request();
        request.get().set_line(line);
        request.get().set_column(column);
        request.get().set_length(length);

        self.core
            .run(request.send().promise)
            .chain_err(|| "unable to run event loop")?;
        Ok(())
    }

    pub fn replace(mut self, line: u64, column: u64, length: u64, character: char) -> Result<()> {
        let mut request = self.editor.delete_request();
        request.get().set_line(line);
        request.get().set_column(column);
        request.get().set_length(length);

        self.core
            .run(request.send().promise)
            .chain_err(|| "unable to run event loop")?;

        let mut replacement = String::new();
        for _ in 0..length {
            replacement.push(character);
        }

        let mut request = self.editor.insert_request();
        request.get().set_line(line);
        request.get().set_column(column);
        request.get().set_string(&replacement);

        self.core
            .run(request.send().promise)
            .chain_err(|| "unable to run event loop")?;

        Ok(())
    }

    pub fn quit(mut self) -> Result<()> {
        let request = self.editor.quit_request();

        self.core
            .run(request.send().promise)
            .chain_err(|| "unable to run event loop")?;
        Ok(())
    }
}
