extern crate clap;
extern crate rustty;

mod editor;
mod file;
mod terminal;

use clap::App;

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
        let mut editor = editor::Editor::new(file.to_string());
        terminal::run(&mut editor);
    } else {
        let mut editor = editor::Editor::new("".to_string());
        terminal::run(&mut editor);
    }
}
