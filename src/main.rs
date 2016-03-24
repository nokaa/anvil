/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

extern crate clap;
extern crate crypto;
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
                       .arg_from_usage("[FILE] 'The file to be edited'")
                       .get_matches();

    // If the user passed a file as an arg, we create a new editor with
    // it and run the UI. Otherwise, we create a new editor with a blank
    // filename and run the UI.
    if let Some(file) = matches.value_of("FILE") {
        let mut editor = editor::Editor::new(file);
        let mut term = term::Term::new(&mut editor);
        term.run();
    } else {
        let mut editor = editor::Editor::new("");
        let mut term = term::Term::new(&mut editor);
        term.run();
    }
}
