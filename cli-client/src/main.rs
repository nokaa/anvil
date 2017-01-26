#![recursion_limit = "1024"]

extern crate anvil_rpc;
extern crate anvil_server;
extern crate capnp;
#[macro_use]
extern crate capnp_rpc;
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate tokio_core;
extern crate tokio_uds;

mod commands;

use clap::{Arg, App, SubCommand};

use std::u64;

fn main() {
    let matches = App::new("anvil-cli-client")
        .version("0.1")
        .author("nokaa <nokaa@cock.li>")
        .about("A CLI client for making RPC calls on an Anvil server")
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("PATH")
            .help("Sets path of the UDS used by the server")
            .takes_value(true)
            .required(false))
        .subcommand(SubCommand::with_name("insert")
            .about("Insert <TEXT> at [LINE][COLUMN]")
            .arg(Arg::with_name("LINE")
                .help("The line number to insert at. Lines start at 0.")
                .required(true)
                .index(1))
            .arg(Arg::with_name("COLUMN")
                .help("The column to insert at. Columns start at 0.")
                .required(true)
                .index(2))
            .arg(Arg::with_name("TEXT")
                .help("The text to be inserted")
                .required(true)
                .index(3)))
        .get_matches();

    let path = matches.value_of("path").unwrap_or("\0anvil_uds");
    if let Some(matches) = matches.subcommand_matches("insert") {
        let text = matches.value_of("TEXT").unwrap();
        let line = matches.value_of("LINE").unwrap();
        let column = matches.value_of("COLUMN").unwrap();
        let line = u64::from_str_radix(line, 10).unwrap();
        let column = u64::from_str_radix(column, 10).unwrap();

        let cmd = commands::Command::new(path).unwrap();
        cmd.insert(line, column, text).unwrap();
    }
}
