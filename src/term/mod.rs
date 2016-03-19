/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

mod command;
mod cursor;
mod insert;

use rustty::{Terminal, Event, Color};
use editor;

pub struct Term<'a> {
    cursor: cursor::Cursor,
    editor: &'a mut editor::Editor<'a>,
    term: Terminal,
    pub quit: bool,
}

impl<'a> Term<'a> {
    pub fn new(editor: &'a mut editor::Editor<'a>) -> Term<'a> {
        Term {
            cursor: cursor::Cursor::new(Color::Red),
            editor: editor,
            term: Terminal::new().unwrap(),
            quit: false,
        }
    }

/// Launches the terminal
    pub fn run(&mut self) {
        self.term[self.cursor.current_pos()].set_bg(self.cursor.color);
        self.print_file();
        self.prompt();
        self.term.swap_buffers().unwrap();

        while !self.quit {
            let evt = self.term.get_event(100).unwrap();
            if let Some(Event::Key(ch)) = evt {
                if self.editor.command_mode() {
                    command::handle(self, ch);
                } else {
                    insert::handle(self, ch);
                }

                if self.cursor.pos.x >= self.term.cols() - 1 {
                    self.term[self.cursor.last_pos()].set_bg(Color::Default);
                    self.cursor.save_pos();
                    self.cursor.pos.x = 0;
                    self.cursor.pos.y += 1;
                }
                if self.cursor.pos.y >= self.term.rows() - 1 {
                    self.term[self.cursor.last_pos()].set_bg(Color::Default);
                    self.cursor.save_pos();
                    self.cursor.pos.x = 0;
                    self.cursor.pos.y = 0;
                }

                self.term[self.cursor.last_pos()].set_bg(Color::Default);
                self.term[self.cursor.current_pos()].set_bg(self.cursor.color);
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
