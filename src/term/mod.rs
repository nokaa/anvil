/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

mod command;
mod cursor;
mod insert;

use rustty::{self, Event, Color};
use editor;

/// `Term` represents our client application. This allows
/// us to work with the filesystem and the UI.
pub struct Term<'a> {
    /// Represents the location of the cursor in our UI
    cursor: cursor::Cursor,
    /// Contains file information; allows us to work with
    /// the filesystem
    editor: &'a mut editor::Editor<'a>,
    /// This is the UI itself
    term: rustty::Terminal,
    /// Represents the running status
    quit: bool,
    /// Represents the topmost line in our UI
    line: usize,
    /// Represents the total number of lines in the currently
    /// open file
    total_lines: usize,
}

impl<'a> Term<'a> {
    pub fn new(editor: &'a mut editor::Editor<'a>) -> Term<'a> {
        let total_lines = editor.contents.len();
        Term {
            cursor: cursor::Cursor::new(Color::Red),
            editor: editor,
            term: rustty::Terminal::new().unwrap(),
            quit: false,
            line: 0,
            total_lines: total_lines,
        }
    }

    /// Launches the terminal.
    pub fn run(&mut self) {
        self.term[self.cursor.current_pos()].set_bg(self.cursor.color);
        let current_line = self.current_line();
        self.print_file(current_line);
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

    /// Prints our prompt to the UI.
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

    /// Prints our editor's contents to the UI.
    fn print_file(&mut self, start: usize) {
        let w = self.term.cols();
        let mut i = 0;
        let mut j = 0;
        for line in &self.editor.contents {
            if i < start {
                i += 1;
                continue;
            }

            for b in line {
                self.term[(j, i - start)].set_ch(*b as char);
                j += 1;
                // If the current line is longer than term width, we move
                // to a new line.
                //
                // TODO(nokaa): If line-width is greater than term width,
                // we should insert a new line our editor array
                if j == w {
                    j = 0;
                    i += 1;
                }
            }
            while j < w {
                self.term[(j, i - start)].set_ch(' ');
                j += 1;
            }

            i += 1;
            j = 0;
            if i - start == self.term.rows() - 2 {
                break;
            }
        }
    }

    pub fn move_cursor(&mut self, direction: cursor::Direction) {
        match direction {
            cursor::Direction::Up => {
                if self.cursor.pos.y != 0 {
                    self.cursor.save_pos();
                    self.cursor.pos.y -= 1;
                } else if self.current_line() != 0 {
                    let current_line = self.current_line();
                    self.set_current_line(current_line - 1);
                    self.redraw_file();
                }
            }
            cursor::Direction::Down => {
                if self.total_lines() <= self.term.rows() - 3 {
                    if self.cursor.pos.y != self.term.rows() - 3 &&
                       self.cursor.pos.y < self.total_lines() - 1
                    {
                        self.cursor.save_pos();
                        self.cursor.pos.y += 1;
                    }
                } else {
                    if self.cursor.pos.y != self.term.rows() - 3 {
                        self.cursor.save_pos();
                        self.cursor.pos.y += 1;
                    } else if self.cursor.pos.y + self.current_line() <
                              self.total_lines() - 1
                    {
                        let current_line = self.current_line();
                        self.set_current_line(current_line + 1);
                        self.redraw_file();
                    }
                }
            }
            cursor::Direction::Left => {
                if self.cursor.pos.x != 0 {
                    self.cursor.save_pos();
                    self.cursor.pos.x -= 1;
                }
            }
            cursor::Direction::Right => {
                let curr = self.cursor.pos.y + self.current_line();
                if self.cursor.pos.x != self.editor.contents[curr].len() - 1 {
                    self.cursor.save_pos();
                    self.cursor.pos.x += 1;
                }
            }
        }
    }

    /// Changes the value of the `quit` attribute to `true`.
    pub fn quit(&mut self) {
        self.quit = true;
    }

    /// Returns the total lines in our file
    pub fn total_lines(&self) -> usize {
        self.total_lines
    }

    /// Allows us to modify the total number of lines
    /// in our file, e.g. if a new line is added.
    pub fn set_total_lines(&mut self, change: isize) {
        let new = self.total_lines as isize + change;
        self.total_lines = new as usize;
    }

    /// Gives the current line at the top of our UI
    pub fn current_line(&self) -> usize {
        self.line
    }

    /// Allows us to change what the topmost line in the UI is
    pub fn set_current_line(&mut self, line: usize) {
        self.line = line;
    }
    
    /// This function redraws the portion of our UI displaying
    /// file contents. E.g. this is called when the current line
    /// changes.
    pub fn redraw_file(&mut self) {
        let current_line = self.current_line();
        self.print_file(current_line);
    }
}
