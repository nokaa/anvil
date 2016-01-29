extern crate clap;
extern crate mio;

use clap::App;
use mio::*;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    // Clap handles command line args for us.
    let matches = App::new("forge")
                      .version("0.1")
                      .author("nokaa <nokaa@cock.li>")
                      .about("A text editor")
                      .arg_from_usage("[OUTPUT] 'Sets the output file to use'")
                      .get_matches();

    // For now we are just reading from stdin, so we check to see if the user passed a file to
    // output to. Otherwise we output to stdout
    if let Some(file) = matches.value_of("OUTPUT") {
        let mut input = Vec::new();
        if let Err(e) = io::stdin().read_to_end(&mut input) {
            panic!("Error: {}", e);
        }

        if let Err(e) = write_file(file.to_string(), input) {
            panic!("Error: {}", e);
        }
    } else {
        let mut input = Vec::new();
        if let Err(e) = io::stdin().read_to_end(&mut input) {
            panic!("Error: {}", e);
        }
        if let Err(e) = io::stdout().write_all(&input[..]) {
            panic!("Error: {}", e);
        }
    }
}

#[allow(dead_code)]
/// Helper function for reading from a file. Reads from `filename` and returns a `Vec<u8>`.
fn read_file(filename: String) -> Vec<u8> {
    let mut reader = File::open(Path::new(&filename)).ok().expect("Unable to open file");
    let mut buf: Vec<u8> = vec![];
    reader.read_to_end(&mut buf).unwrap();

    buf
}

/// Helper function for writing to a file. Writes `contents` to `filename`.
fn write_file(filename: String, contents: Vec<u8>) -> Result<(), std::io::Error> {
    let mut file = try!(File::create(Path::new(&filename)));
    try!(file.write_all(&contents[..]));
    Ok(())
}
