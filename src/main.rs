extern crate clap;

mod file;

use clap::App;

use std::io::{self, Read, Write};

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

        if let Err(e) = file::write_file(file.to_string(), input) {
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
