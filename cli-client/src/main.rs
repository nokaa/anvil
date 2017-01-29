#![recursion_limit = "1024"]

extern crate anvil_rpc;
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
mod error;

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
        .subcommand(SubCommand::with_name("openFile")
            .about("Opens FILENAME and sets the editors contents to the file's contents")
            .arg(Arg::with_name("FILENAME")
                .help("The filename to read editor contents from.")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("writeFile")
            .about("Save editor contents to FILENAME")
            .arg(Arg::with_name("FILENAME")
                .help("The filename to write editor contents to.")
                .required(true)
                .index(1)))
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
                .help("The text to be inserted.")
                .required(true)
                .index(3)))
        .subcommand(SubCommand::with_name("delete")
            .about("Deletes LENGTH characters starting at [LINE][COLUMN]")
            .arg(Arg::with_name("LINE")
                .help("The line number to delete at. Lines start at 0.")
                .required(true)
                .index(1))
            .arg(Arg::with_name("COLUMN")
                .help("The column to delete at. Columns start at 0.")
                .required(true)
                .index(2))
            .arg(Arg::with_name("LENGTH")
                .help("The number of characters to delete.")
                .required(true)
                .index(3)))
        .subcommand(SubCommand::with_name("replace")
            .about("replaces LENGTH characters with CHARACTER starting at [LINE][COLUMN]")
            .arg(Arg::with_name("LINE")
                .help("The line number to replace at. Lines start at 0.")
                .required(true)
                .index(1))
            .arg(Arg::with_name("COLUMN")
                .help("The column to replace at. Columns start at 0.")
                .required(true)
                .index(2))
            .arg(Arg::with_name("LENGTH")
                .help("The number of characters to replace.")
                .required(true)
                .index(3))
            .arg(Arg::with_name("CHARACTER")
                .help("The character to replace with.")
                .required(true)
                .index(4)))
        .subcommand(SubCommand::with_name("quit").about("Quits this editor"))
        .get_matches();

    let path = matches.value_of("path").unwrap_or("\0anvil_uds");

    if let Some(matches) = matches.subcommand_matches("openFile") {
        let file_name = matches.value_of("FILENAME").unwrap();
        let cmd = commands::Command::new(path).unwrap();
        cmd.open_file(file_name).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("writeFile") {
        let file_name = matches.value_of("FILENAME").unwrap();
        let cmd = commands::Command::new(path).unwrap();
        cmd.write_file(file_name).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("insert") {
        let text = matches.value_of("TEXT").unwrap();
        let line = matches.value_of("LINE").unwrap();
        let column = matches.value_of("COLUMN").unwrap();
        let line = u64::from_str_radix(line, 10).unwrap();
        let column = u64::from_str_radix(column, 10).unwrap();

        let cmd = commands::Command::new(path).unwrap();
        cmd.insert(line, column, text).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        let line = matches.value_of("LINE").unwrap();
        let column = matches.value_of("COLUMN").unwrap();
        let length = matches.value_of("LENGTH").unwrap();
        let line = u64::from_str_radix(line, 10).unwrap();
        let column = u64::from_str_radix(column, 10).unwrap();
        let length = u64::from_str_radix(length, 10).unwrap();

        let cmd = commands::Command::new(path).unwrap();
        cmd.delete(line, column, length).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("replace") {
        let character = matches.value_of("CHARACTER").unwrap();
        let line = matches.value_of("LINE").unwrap();
        let column = matches.value_of("COLUMN").unwrap();
        let length = matches.value_of("LENGTH").unwrap();
        let line = u64::from_str_radix(line, 10).unwrap();
        let column = u64::from_str_radix(column, 10).unwrap();
        let length = u64::from_str_radix(length, 10).unwrap();

        if character.len() > 1 {
            panic!("CHARACTER must be a char, given {}", character);
        }

        let character = character.chars().nth(0).unwrap();

        let cmd = commands::Command::new(path).unwrap();
        cmd.replace(line, column, length, character).unwrap();
    } else if let Some(_matches) = matches.subcommand_matches("quit") {
        let cmd = commands::Command::new(path).unwrap();
        cmd.quit().unwrap();
    }
}
