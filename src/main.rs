extern crate clap;
extern crate rustty;

mod file;
mod editor;

use clap::App;
use rustty::{Terminal, Event, Color};

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
        //let f = file::read_file(file);
        //let text = String::from_utf8(f).unwrap();
        run(&mut editor);
    } else {
        let mut editor = editor::Editor::new("".to_string());
        //let text = "Forge";
        run(&mut editor);
    }
}

struct Cursor {
    pos: Position,
    lpos: Position,
    color: Color,
}

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

fn run(editor: &mut editor::Editor) {
    let mut cursor = Cursor {
        pos: Position {x: 0, y: 0},
        lpos: Position {x: 0, y: 0},
        color: Color::Red,
    };

    let mut term = Terminal::new().unwrap();
    term[(cursor.pos.x, cursor.pos.y)].set_bg(cursor.color);
    term.swap_buffers().unwrap();

    loop {
        let evt = term.get_event(100).unwrap();
        if let Some(Event::Key(ch)) = evt {
            if editor.command_mode() {
                match ch {
                    'i' => {
                        editor.switch_mode();
                    }
                    'q' => {
                        break;
                    }
                    _ => { }
                }
            } else {
                match ch {
                    '\x1b' => {
                        editor.switch_mode();
                    }
                    '\x7f' => {
                        cursor.lpos = cursor.pos;
                        if cursor.pos.x == 0 {
                            cursor.pos.y = cursor.pos.y.saturating_sub(1);
                        } else {
                            cursor.pos.x -= 1;
                        }
                        term[(cursor.pos.x, cursor.pos.y)].set_ch(' ');
                    }
                    '\r' => {
                        cursor.lpos = cursor.pos;
                        cursor.pos.x = 0;
                        cursor.pos.y += 1;
                    }
                    c @ _ => {
                        term[(cursor.pos.x, cursor.pos.y)].set_ch(c);
                        cursor.lpos = cursor.pos;
                        cursor.pos.x += 1;
                    }
                }
            }

            if cursor.pos.x >= term.cols() - 1 {
                term[(cursor.lpos.x, cursor.lpos.y)].set_bg(Color::Default);
                cursor.lpos = cursor.pos;
                cursor.pos.x = 0;
                cursor.pos.y += 1;
            }
            if cursor.pos.y >= term.rows() - 1 {
                term[(cursor.lpos.x, cursor.lpos.y)].set_bg(Color::Default);
                cursor.lpos = cursor.pos;
                cursor.pos.x = 0;
                cursor.pos.y = 0;
            }

            term[(cursor.lpos.x, cursor.lpos.y)].set_bg(Color::Default);
            term[(cursor.pos.x, cursor.pos.y)].set_bg(cursor.color);
            term.swap_buffers().unwrap();
        }
    }
}
