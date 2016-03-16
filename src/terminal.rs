use rustty::{Terminal, Event, Color};
use editor;

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

struct Cursor {
    pos: Position,
    lpos: Position,
    color: Color,
}

pub fn run(editor: &mut editor::Editor) {
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
                    'h' => {
                        if cursor.pos.x != 0 {
                            cursor.pos.x -= 1;
                        }
                    }
                    'j' => {
                        // TODO(nokaa): We don't want to go beyond
                        // the working area here.
                        cursor.pos.y += 1;

                    }
                    'k' => {
                        if cursor.pos.y != 0 {
                            cursor.pos.y -= 1;
                        }
                    }
                    'l' => {
                        // TODO(nokaa): We don't want to extend beyond the
                        // line length here, but we first need a way to
                        // determine a given line's length.
                        cursor.pos.x += 1;
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
                    '\t' => {
                        term[(cursor.pos.x, cursor.pos.y)].set_ch(' ');
                        term[(cursor.pos.x + 1, cursor.pos.y)].set_ch(' ');
                        term[(cursor.pos.x + 2, cursor.pos.y)].set_ch(' ');
                        term[(cursor.pos.x + 3, cursor.pos.y)].set_ch(' ');
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
