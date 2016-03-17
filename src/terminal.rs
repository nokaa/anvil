/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

use rustty::{Terminal, Event, Color};
use editor;

/// Represents an `x, y` coordinate.
#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

/// Represents the cursor in our terminal
struct Cursor {
    pos: Position,
    lpos: Position,
    color: Color,
}

/// Launches the terminal using the given `editor`.
pub fn run(editor: &mut editor::Editor) {
    let mut cursor = Cursor {
        pos: Position {x: 0, y: 0},
        lpos: Position {x: 0, y: 0},
        color: Color::Red,
    };

    let mut term = Terminal::new().unwrap();
    term[(cursor.pos.x, cursor.pos.y)].set_bg(cursor.color);
    print_file(&mut term, &editor.contents);
    prompt(&mut term, editor.filename());
    term.swap_buffers().unwrap();

    loop {
        let evt = term.get_event(100).unwrap();
        if let Some(Event::Key(ch)) = evt {
            if editor.command_mode() {
                match ch {
                    'i' => {
                        editor.switch_mode();
                    }
                    'h' => {
                        if cursor.pos.x != 0 {
                            cursor.lpos = cursor.pos;
                            cursor.pos.x -= 1;
                        }
                    }
                    'j' => {
                        // TODO(nokaa): We don't want to go beyond
                        // the working area here.
                        cursor.lpos = cursor.pos;
                        cursor.pos.y += 1;

                    }
                    'k' => {
                        if cursor.pos.y != 0 {
                            cursor.lpos = cursor.pos;
                            cursor.pos.y -= 1;
                        }
                    }
                    'l' => {
                        // TODO(nokaa): We don't want to extend beyond the
                        // line length here, but we first need a way to
                        // determine a given line's length.
                        cursor.lpos = cursor.pos;
                        cursor.pos.x += 1;
                    }
                    'q' => {
                        break;
                    }
                    _ => { }
                }
            } else {
                match ch {
                    '\x1b' => { // Escape key
                        editor.switch_mode();
                    }
                    '\x7f' => { // Backspace key
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
                    '\t' => {
                        for i in 0..4 {
                            term[(cursor.pos.x + i, cursor.pos.y)].set_ch(' ');
                        }
                        cursor.lpos = cursor.pos;
                        cursor.pos.x += 4;
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

fn prompt(term: &mut Terminal, filename: &str) {
    let w = term.cols();
    let h = term.rows();

    for i in 0..w {
        term[(i, h - 2)].set_bg(Color::Red);
    }

    for (i, c) in filename.chars().enumerate() {
        term[(i, h - 2)].set_ch(c);
    }
}

fn print_file(term: &mut Terminal, contents: &Vec<Vec<u8>>) {
    let mut i = 0;
    let mut j = 0;
    for line in contents {
        for b in line {
            term[(j, i)].set_ch(*b as char);
            j += 1;
        }
        i += 1;
        j = 0;
        if i == term.rows() - 2 {
            break;
        }
    }
}
