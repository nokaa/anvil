use error::*;
use anvil_rpc::editor;

use capnp::capability::Promise;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use futures::{Future, Stream};
use tokio_core::io::Io;
use tokio_core::reactor::Core;
use tokio_uds::UnixListener;
use xi_rope::Rope;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

struct EditorImpl {
    content: Rope,
}

impl EditorImpl {
    pub fn new() -> EditorImpl {
        EditorImpl { content: Rope::from("") }
    }
}

impl editor::Server for EditorImpl {
    fn open_file(&mut self,
                 params: editor::OpenFileParams,
                 _results: editor::OpenFileResults)
                 -> Promise<(), ::capnp::Error> {
        let file_name = pry!(pry!(params.get()).get_path());
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_name)
            .unwrap();

        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        let content = Rope::from(buf);

        self.content = content;

        Promise::ok(())
    }

    fn write_file(&mut self,
                  params: editor::WriteFileParams,
                  _results: editor::WriteFileResults)
                  -> Promise<(), ::capnp::Error> {
        let file_name = pry!(pry!(params.get()).get_path());
        let mut file = File::create(file_name).unwrap();

        file.write_all(String::from(self.content.clone()).as_bytes()).unwrap();

        Promise::ok(())
    }

    fn insert(&mut self,
              params: editor::InsertParams,
              _results: editor::InsertResults)
              -> Promise<(), ::capnp::Error> {
        let line = pry!(params.get()).get_line() as usize;
        let column = pry!(params.get()).get_column() as usize;
        let text = pry!(pry!(params.get()).get_string());

        {
            let index = self.content.offset_of_line(line) + column;
            self.content.edit_str(index, index, text)
        }

        println!("{}", String::from(self.content.clone()));
        Promise::ok(())
    }

    fn delete(&mut self,
              params: editor::DeleteParams,
              _results: editor::DeleteResults)
              -> Promise<(), ::capnp::Error> {
        let line = pry!(params.get()).get_line() as usize;
        let column = pry!(params.get()).get_column() as usize;
        let length = pry!(params.get()).get_length() as usize;

        {
            let index = self.content.offset_of_line(line) + column;
            self.content.edit_str(index, index + length, "");
        }

        println!("{}", String::from(self.content.clone()));
        Promise::ok(())
    }

    fn quit(&mut self,
            _params: editor::QuitParams,
            _results: editor::QuitResults)
            -> Promise<(), ::capnp::Error> {
        println!("Quit request");
        Promise::ok(())
    }
}

pub fn server<P>(path: P) -> Result<()>
    where P: AsRef<Path>
{
    let mut core = Core::new().chain_err(|| "unable to create event loop")?;
    let handle = core.handle();

    let socket = UnixListener::bind(path, &handle).chain_err(|| "unable to bind to UDS")?;

    let editor_impl = EditorImpl::new();

    let editor = editor::ToClient::new(editor_impl).from_server::<::capnp_rpc::Server>();

    let handle1 = handle.clone();
    let done = socket.incoming()
        .for_each(move |(stream, _addr)| {
            let (reader, writer) = stream.split();
            let handle = handle1.clone();

            let network = twoparty::VatNetwork::new(reader,
                                                    writer,
                                                    rpc_twoparty_capnp::Side::Server,
                                                    Default::default());

            let rpc_system = RpcSystem::new(Box::new(network), Some(editor.clone().client));

            handle.spawn(rpc_system.map_err(|_| ()));
            Ok(())
        });

    core.run(done).chain_err(|| "unable to run event loop")?;
    Ok(())
}
