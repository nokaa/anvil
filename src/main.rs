extern crate clap;
extern crate rustbox;

mod file;

use clap::App;
use rustbox::{Color, Key, RustBox};

use std::default::Default;
//use std::io::{self, Read, Write};

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
        let f = file::read_file(file);
        let text = String::from_utf8(f).unwrap();
        run(&text[..]);
        /*let mut input = Vec::new();
        if let Err(e) = io::stdin().read_to_end(&mut input) {
            panic!("Error: {}", e);
        }

        if let Err(e) = file::write_file(file.to_string(), input) {
            panic!("Error: {}", e);
        }*/
    } else {
        let text = "Forge";
        run(text);
        /*let mut input = Vec::new();
        if let Err(e) = io::stdin().read_to_end(&mut input) {
            panic!("Error: {}", e);
        }
        if let Err(e) = io::stdout().write_all(&input[..]) {
            panic!("Error: {}", e);
        }*/
    }
}

fn run(text: &str) {
    let rustbox = match RustBox::init(Default::default()) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let mut lines = text.lines();
    let mut y = 1;
    while let Some(line) = lines.next() {
        rustbox.print(1, y, rustbox::RB_NORMAL, Color::White, Color::Black, line);
        y += 1;
    }

    loop {
        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}
