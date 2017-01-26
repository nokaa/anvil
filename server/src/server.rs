use error::*;
use anvil_rpc::{editor, plugin, subscription};

use capnp::capability::Promise;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use futures::{Future, Stream};
use nix::unistd;
use tokio_core::io::Io;
use tokio_core::reactor::{self, Core};
use tokio_uds::UnixListener;
// use xi_rope::Rope;

use std::cell::RefCell;
use std::collections::HashMap;
// use std::io::{self, Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::time;

struct PluginHandle {
    client: plugin::Client<::capnp::text::Owned>,
    requests_in_flight: i32,
}

struct PluginMap {
    subscribers: HashMap<u64, PluginHandle>,
}

impl PluginMap {
    fn new() -> PluginMap {
        PluginMap { subscribers: HashMap::new() }
    }
}

struct SubscriptionImpl {
    id: u64,
    subscribers: Rc<RefCell<PluginMap>>,
}

impl SubscriptionImpl {
    fn new(id: u64, subscribers: Rc<RefCell<PluginMap>>) -> Self {
        SubscriptionImpl {
            id: id,
            subscribers: subscribers,
        }
    }
}

impl Drop for SubscriptionImpl {
    fn drop(&mut self) {
        println!("subscription dropped");
        self.subscribers.borrow_mut().subscribers.remove(&self.id);
    }
}

impl subscription::Server for SubscriptionImpl {}

struct EditorImpl {
    next_id: u64,
    subscribers: Rc<RefCell<PluginMap>>,
    content: Vec<String>,
}

impl EditorImpl {
    pub fn new() -> (EditorImpl, Rc<RefCell<PluginMap>>) {
        let subscribers = Rc::new(RefCell::new(PluginMap::new()));
        (EditorImpl {
             next_id: 0,
             subscribers: subscribers.clone(),
             content: vec![String::new()],
         },
         subscribers.clone())
    }
}

impl editor::Server<::capnp::text::Owned> for EditorImpl {
    fn subscribe(&mut self,
                 params: editor::SubscribeParams<::capnp::text::Owned>,
                 mut results: editor::SubscribeResults<::capnp::text::Owned>)
                 -> Promise<(), ::capnp::Error> {
        println!("subscribe");
        self.subscribers
            .borrow_mut()
            .subscribers
            .insert(self.next_id,
                    PluginHandle {
                        client: pry!(pry!(params.get()).get_plugin()),
                        requests_in_flight: 0,
                    });

        results.get()
            .set_subscription(subscription::ToClient::new(SubscriptionImpl::new(self.next_id,
                                                                                self.subscribers
                                                                                    .clone()))
                .from_server::<::capnp_rpc::Server>());

        self.next_id += 1;
        Promise::ok(())
    }

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
    // Unlink the UDS if it already exists.
    // unistd::unlink(path.as_ref())?;

    let mut core = Core::new().chain_err(|| "unable to create event loop")?;
    let handle = core.handle();

    let socket = UnixListener::bind(path, &handle).chain_err(|| "unable to bind to UDS")?;

    let (publisher_impl, subscribers) = EditorImpl::new();

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
        })
        .map_err(|e| e.into());

    let infinite = ::futures::stream::iter(::std::iter::repeat(()).map(Ok::<(), Error>));
    let send_to_subscribers = infinite.fold((handle, subscribers),
                                            move |(handle, subscribers), ()|
           -> Promise<(::tokio_core::reactor::Handle, Rc<RefCell<PluginMap>>), Error> {
        {
            let subscribers1 = subscribers.clone();
            let subs = &mut subscribers.borrow_mut().subscribers;
            for (&idx, mut subscriber) in subs.iter_mut() {
                if subscriber.requests_in_flight < 5 {
                    subscriber.requests_in_flight += 1;
                    let mut request = subscriber.client.push_message_request();
                    pry!(request.get().set_message(
                        &format!("system time is: {:?}", ::std::time::SystemTime::now())[..]));

                    let subscribers2 = subscribers1.clone();
                    handle.spawn(request.send().promise.then(move |r| {
                        match r {
                            Ok(_) => {
                                subscribers2.borrow_mut()
                                    .subscribers.get_mut(&idx)
                                    .map(|ref mut s| {
                                        s.requests_in_flight -= 1;
                                });
                            }
                            Err(e) => {
                                println!("Got error: {:?}. Dropping subscriber.", e);
                                subscribers2.borrow_mut().subscribers.remove(&idx);
                            }
                        }
                        Ok::<(), Error>(())
                    }).map_err(|_| unreachable!()));
                }
            }
        }

        let timeout = pry!(reactor::Timeout::new(time::Duration::from_secs(1), &handle));
        let timeout = timeout.and_then(move |()| Ok((handle, subscribers))).map_err(|e| e.into());
        Promise::from_future(timeout)
    });

    core.run(send_to_subscribers.join(done)).chain_err(|| "unable to run event loop")?;
    Ok(())
}
