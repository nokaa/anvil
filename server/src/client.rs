use error::*;
use rpc_capnp::{editor, plugin};

use capnp::capability::Promise;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use futures::Future;
use tokio_core::reactor::Core;
use tokio_core::io::Io;
use tokio_uds::UnixStream;

use std::path::Path;

struct PluginImpl;

impl plugin::Server<::capnp::text::Owned> for PluginImpl {
    fn push_message(&mut self,
                    params: plugin::PushMessageParams<::capnp::text::Owned>,
                    _results: plugin::PushMessageResults<::capnp::text::Owned>)
                    -> Promise<(), ::capnp::Error> {
        println!("message from publisher: {}",
                 pry!(pry!(params.get()).get_message()));
        Promise::ok(())
    }
}

pub fn client<P>(path: P) -> Result<()>
    where P: AsRef<Path>
{
    let mut core = Core::new().chain_err(|| "unable to create event loop")?;
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

    // let sub = plugin::ToClient::new(PluginImpl).from_server::<::capnp_rpc::Server>();

    // let mut request = editor.subscribe_request();
    // request.get().set_plugin(sub);

    let mut request = editor.insert_request();
    request.get().set_line(0);
    request.get().set_column(0);
    request.get().set_string("some string");

    // Need to make sure not to drop the returned subscription object.
    let _result = core.run(rpc_system.join(request.send().promise))
        .chain_err(|| "unable to run event loop")?;
    Ok(())
}
