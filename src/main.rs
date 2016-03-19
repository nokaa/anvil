/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

extern crate clap;
extern crate rustty;

mod editor;
mod file;
mod term;

use clap::App;

fn main() {
    // Clap handles command line args for us.
    let matches = App::new("forge")
                       .version("v0.1")
                       .author("nokaa <nokaa@cock.li>")
                       .about("A text editor")
                       .arg_from_usage("[OUTPUT] 'Sets the output file to use'")
                       .get_matches();

    // For now we are just reading from stdin, so we check to see if the
    // user passed a file to output to. Otherwise we output to stdout.
    if let Some(file) = matches.value_of("OUTPUT") {
        let mut editor = editor::Editor::new(file);
        let mut term = term::Term::new(&mut editor);
        term.run();
    } else {
        let mut editor = editor::Editor::new("");
        let mut term = term::Term::new(&mut editor);
        term.run();
    }
}
