use error::*;
use anvil_rpc::editor;

use capnp::capability::Promise;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use futures::{Future, Stream};
use tokio_core::io::Io;
use tokio_core::reactor::Core;
use tokio_uds::UnixListener;
// use xi_rope::Rope;

use std::path::Path;

struct EditorImpl {
    content: Vec<String>,
}

impl EditorImpl {
    pub fn new() -> EditorImpl {
        EditorImpl { content: vec![String::new()] }
    }
}

impl editor::Server<::capnp::text::Owned> for EditorImpl {
    fn insert(&mut self,
              params: editor::InsertParams<::capnp::text::Owned>,
              _results: editor::InsertResults<::capnp::text::Owned>)
              -> Promise<(), ::capnp::Error> {
        let line = pry!(params.get()).get_line();
        let column = pry!(params.get()).get_column();
        let text = pry!(pry!(params.get()).get_string());

        {
            let mut line = &mut self.content[line as usize];
            line.insert_str(column as usize, text);
        }
        println!("{:?}", self.content);
        Promise::ok(())
    }

    fn write_file(&mut self,
                  _params: editor::WriteFileParams<::capnp::text::Owned>,
                  _results: editor::WriteFileResults<::capnp::text::Owned>)
                  -> Promise<(), ::capnp::Error> {
        Promise::ok(())
    }
}

pub fn server<P>(path: P) -> Result<()>
    where P: AsRef<Path>
{
    let mut core = Core::new().chain_err(|| "unable to create event loop")?;
    let handle = core.handle();

    let socket = UnixListener::bind(path, &handle).chain_err(|| "unable to bind to UDS")?;

    let publisher_impl = EditorImpl::new();

    let publisher = editor::ToClient::new(publisher_impl).from_server::<::capnp_rpc::Server>();

    let handle1 = handle.clone();
    let done = socket.incoming()
        .for_each(move |(stream, _addr)| {
            let (reader, writer) = stream.split();
            let handle = handle1.clone();

            let network = twoparty::VatNetwork::new(reader,
                                                    writer,
                                                    rpc_twoparty_capnp::Side::Server,
                                                    Default::default());

            let rpc_system = RpcSystem::new(Box::new(network), Some(publisher.clone().client));

            handle.spawn(rpc_system.map_err(|_| ()));
            Ok(())
        });

    core.run(done).chain_err(|| "unable to run event loop")?;
    Ok(())
}
