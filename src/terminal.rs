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

pub struct Term<'a> {
    cursor: Cursor,
    editor: &'a mut editor::Editor<'a>,
    term: Terminal,
}

impl<'a> Term<'a> {
    pub fn new(editor: &'a mut editor::Editor<'a>) -> Term<'a> {
        Term {
            cursor: Cursor {
                pos: Position {x: 0, y: 0},
                lpos: Position {x: 0, y: 0},
                color: Color::Red,
            },
            editor: editor,
            term: Terminal::new().unwrap(),
        }
    }

/// Launches the terminal
    pub fn run(&mut self) {
        self.term[(self.cursor.pos.x, self.cursor.pos.y)].set_bg(self.cursor.color);
        self.print_file();
        self.prompt();
        self.term.swap_buffers().unwrap();

        loop {
            let evt = self.term.get_event(100).unwrap();
            if let Some(Event::Key(ch)) = evt {
                if self.editor.command_mode() {
                    match ch {
                        'i' => {
                            self.editor.switch_mode();
                        }
                        'h' => {
                            if self.cursor.pos.x != 0 {
                                self.cursor.lpos = self.cursor.pos;
                                self.cursor.pos.x -= 1;
                            }
                        }
                        'j' => {
                            // TODO(nokaa): We don't want to go beyond
                            // the working area here.
                            self.cursor.lpos = self.cursor.pos;
                            self.cursor.pos.y += 1;

                        }
                        'k' => {
                            if self.cursor.pos.y != 0 {
                                self.cursor.lpos = self.cursor.pos;
                                self.cursor.pos.y -= 1;
                            }
                        }
                        'l' => {
                            // TODO(nokaa): We don't want to extend beyond the
                            // line length here, but we first need a way to
                            // determine a given line's length.
                            self.cursor.lpos = self.cursor.pos;
                            self.cursor.pos.x += 1;
                        }
                        'q' => {
                            break;
                        }
                        _ => { }
                    }
                } else {
                    match ch {
                        '\x1b' => { // Escape key
                            self.editor.switch_mode();
                        }
                        '\x7f' => { // Backspace key
                            self.cursor.lpos = self.cursor.pos;
                            if self.cursor.pos.x == 0 {
                                self.cursor.pos.y = self.cursor.pos.y.saturating_sub(1);
                            } else {
                                self.cursor.pos.x -= 1;
                            }
                            self.term[(self.cursor.pos.x, self.cursor.pos.y)].set_ch(' ');
                        }
                        '\r' => {
                            self.cursor.lpos = self.cursor.pos;
                            self.cursor.pos.x = 0;
                            self.cursor.pos.y += 1;
                        }
                        '\t' => {
                            for i in 0..4 {
                                self.term[(self.cursor.pos.x + i, self.cursor.pos.y)].set_ch(' ');
                            }
                            self.cursor.lpos = self.cursor.pos;
                            self.cursor.pos.x += 4;
                        }
                        c @ _ => {
                            self.term[(self.cursor.pos.x, self.cursor.pos.y)].set_ch(c);
                            self.cursor.lpos = self.cursor.pos;
                            self.cursor.pos.x += 1;
                        }
                    }
                }

                if self.cursor.pos.x >= self.term.cols() - 1 {
                    self.term[(self.cursor.lpos.x, self.cursor.lpos.y)].set_bg(Color::Default);
                    self.cursor.lpos = self.cursor.pos;
                    self.cursor.pos.x = 0;
                    self.cursor.pos.y += 1;
                }
                if self.cursor.pos.y >= self.term.rows() - 1 {
                    self.term[(self.cursor.lpos.x, self.cursor.lpos.y)].set_bg(Color::Default);
                    self.cursor.lpos = self.cursor.pos;
                    self.cursor.pos.x = 0;
                    self.cursor.pos.y = 0;
                }

                self.term[(self.cursor.lpos.x, self.cursor.lpos.y)].set_bg(Color::Default);
                self.term[(self.cursor.pos.x, self.cursor.pos.y)].set_bg(self.cursor.color);
                self.term.swap_buffers().unwrap();
            }
        }
    }

    /// Prints our prompt to the UI
    pub fn prompt(&mut self) {
        let w = self.term.cols();
        let h = self.term.rows();

        for i in 0..w {
            self.term[(i, h - 2)].set_bg(Color::Red);
        }

        for (i, c) in self.editor.filename().chars().enumerate() {
            self.term[(i, h - 2)].set_ch(c);
        }
    }

    /// Prints our editor's contents to the UI
    fn print_file(&mut self) {
        let mut i = 0;
        let mut j = 0;
        for line in &self.editor.contents {
            for b in line {
                self.term[(j, i)].set_ch(*b as char);
                j += 1;
            }
            i += 1;
            j = 0;
            if i == self.term.rows() - 2 {
                break;
            }
        }
    }
}
