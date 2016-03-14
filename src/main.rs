extern crate clap;
extern crate rotor;

mod file;

use clap::App;
use rotor::{EventSet, PollOpt, Loop, Config, Void};
use rotor::mio::{TryRead, TryWrite};
use rotor::mio::tcp::{TcpListener, TcpStream};
use rotor::{Machine, Response, EarlyScope, Scope};

use std::io::{self, Write};
use std::net::SocketAddr;

fn main() {
    // Clap handles command line args for us.
    let _matches = App::new("forge")
                       .version("0.1")
                       .author("nokaa <nokaa@cock.li>")
                       .about("A text editor")
                       .arg_from_usage("[OUTPUT] 'Sets the output file to use'")
                       .get_matches();

    run("0.0.0.0:3000".parse().unwrap());

    // For now we are just reading from stdin, so we check to see if the user passed a file to
    // output to. Otherwise we output to stdout
    //    if let Some(file) = matches.value_of("OUTPUT") {
    // let mut input = Vec::new();
    // if let Err(e) = io::stdin().read_to_end(&mut input) {
    // panic!("Error: {}", e);
    // }
    //
    // if let Err(e) = write_file(file.to_string(), input) {
    // panic!("Error: {}", e);
    // }
    // } else {
    // let mut input = Vec::new();
    // if let Err(e) = io::stdin().read_to_end(&mut input) {
    // panic!("Error: {}", e);
    // }
    // if let Err(e) = io::stdout().write_all(&input[..]) {
    // panic!("Error: {}", e);
    // }
    // }

}

/// This function takes a SocketAddr and starts the rotor event loop
/// with on this address.
fn run(address: SocketAddr) {
    let mut loop_creator = Loop::new(&Config::new()).unwrap();
    let lst = TcpListener::bind(&address).unwrap();
    loop_creator.add_machine_with(|scope| Echo::new(lst, scope))
                .unwrap();
    loop_creator.run(Context).unwrap();
}

struct Context;

enum Echo {
    Server(TcpListener),
    Connection(TcpStream),
}

impl Echo {
    pub fn new(sock: TcpListener, scope: &mut EarlyScope) -> Response<Echo, Void> {
        scope.register(&sock, EventSet::readable(), PollOpt::edge())
             .unwrap();
        Response::ok(Echo::Server(sock))
    }

    fn accept(self) -> Response<Echo, TcpStream> {
        match self {
            Echo::Server(sock) => {
                match sock.accept() {
                    Ok(Some((conn, _))) => Response::spawn(Echo::Server(sock), conn),
                    Ok(None) => Response::ok(Echo::Server(sock)),
                    Err(e) => {
                        writeln!(&mut io::stderr(), "Error: {}", e).ok();
                        Response::ok(Echo::Server(sock))
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Machine for Echo {
    type Context = Context;
    type Seed = TcpStream;

    fn create(conn: TcpStream, scope: &mut Scope<Context>) -> Response<Self, Void> {
        scope.register(&conn, EventSet::readable(), PollOpt::level())
             .unwrap();
        Response::ok(Echo::Connection(conn))
    }

    fn ready(self, _events: EventSet, _scope: &mut Scope<Context>) -> Response<Self, TcpStream> {
        match self {
            me @ Echo::Server(..) => me.accept(),
            Echo::Connection(mut sock) => {
                let mut data = [0u8; 1024];
                match sock.try_read(&mut data) {
                    Ok(Some(0)) => Response::done(),
                    Ok(Some(x)) => {
                        // Take the received data and write to file.
                        let input = file::get_nonzero_bytes(&data[..]);
                        let _ = file::write_file("test".to_string(), input);
                        // Write received data to console for testing purposes
                        let _ = io::stdout().write_all(&data[..]);
                        match sock.try_write(&data[..x]) {
                            Ok(_) => {
                                // This is example so we don't care if not all
                                // (or none at all) bytes are written
                                Response::ok(Echo::Connection(sock))
                            }
                            Err(e) => {
                                writeln!(&mut io::stderr(), "write: {}", e).ok();
                                Response::done()
                            }
                        }
                    }
                    Ok(None) => Response::ok(Echo::Connection(sock)),
                    Err(e) => {
                        writeln!(&mut io::stderr(), "read: {}", e).ok();
                        Response::done()
                    }
                }
            }
        }
    }

    fn spawned(self, _scope: &mut Scope<Context>) -> Response<Self, TcpStream> {
        match self {
            me @ Echo::Server(..) => me.accept(),
            _ => unreachable!(),
        }
    }

    fn timeout(self, _scope: &mut Scope<Context>) -> Response<Self, TcpStream> {
        unreachable!();
    }

    fn wakeup(self, _scope: &mut Scope<Context>) -> Response<Self, TcpStream> {
        unreachable!();
    }
}
