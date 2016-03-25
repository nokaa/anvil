/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

mod command;
mod cursor;
mod insert;
mod normal;

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
    /// Represents the topmost line of the file we are editing in our UI
    line: usize,
    /// Represents the total number of lines in the currently
    /// open file
    total_lines: usize,
    partial_lines: Vec<usize>,
}

impl<'a> Term<'a> {
    /// Creates a new `Term` using the given `editor`.
    pub fn new(editor: &'a mut editor::Editor<'a>) -> Term<'a> {
        Term {
            cursor: cursor::Cursor::new(Color::Red),
            editor: editor,
            term: rustty::Terminal::new().unwrap(),
            quit: false,
            line: 0,
            total_lines: 0,
            partial_lines: vec![],
        }
    }

    /// Launches the terminal.
    pub fn run(&mut self) {
        self.editor.read_file();
        let lines = self.editor.contents.len() as isize;
        self.set_total_lines(lines);

        self.term[self.cursor.current_pos()].set_bg(self.cursor.color);
        let current_line = self.current_line();
        self.print_file(current_line);
        self.prompt();
        self.term.swap_buffers().unwrap();

        while !self.quit {
            let evt = self.term.get_event(100).unwrap();
            if let Some(Event::Key(ch)) = evt {
                if self.editor.normal_mode() {
                    normal::handle(self, ch);
                } else if self.editor.command_mode() {
                    command::handle(self, ch);
                } else {
                    insert::handle(self, ch);
                }

                if self.cursor.pos.x > self.term.cols() - 1 {
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
        self.partial_lines = vec![];
        let w = self.term.cols();
        let mut i = 0;
        let mut j = 0;
        for line in &self.editor.contents {
            if i < start {
                i += 1;
                continue;
            }

            for &b in line {
                if b == b'\n' {
                    continue;
                } else if j == w {
                    i += 1;
                    j = 0;
                    self.partial_lines.push(i - start);
                }

                self.term[(j, i - start)].set_ch(b as char);
                j += 1;
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

    /// Handles moving the cursor
    pub fn move_cursor(&mut self, direction: cursor::Direction) {
        match direction {
            cursor::Direction::Up => {
                if self.cursor.pos.y > 0 {
                    self.cursor.save_pos();
                    let mut i = self.cursor.pos.y - 1;
                    while self.partial_lines.contains(&i) {
                        self.cursor.pos.y -= 1;
                        i -= 1;
                    }
                    self.cursor.pos.y -= 1;
                    let curr = self.current_line() - 1;
                    self.set_current_line(curr);
                    
                    let len = self.editor.contents[curr].len() - 1;
                    if len == 0 && self.cursor.pos.x > len {
                        self.cursor.pos.x = len;
                        self.cursor.in_line = len;
                    } else if len > 0 && self.cursor.pos.x > len - 1 {
                        self.cursor.pos.x = len - 1;
                        self.cursor.in_line = len - 1;
                    }
                }
            }
            cursor::Direction::Down => {
                if self.cursor.pos.y < self.term.rows() - 3 {
                    let curr = self.current_line();
                    let len = self.editor.contents[curr].len() - 1;
                    
                    self.cursor.save_pos();
                    self.set_current_line(curr + 1);
                    
                    if len > self.term.cols() {
                        let mut adv = len / self.term.cols();
                        if len % self.term.cols() > 0 {
                            adv += 1;
                        }
                        if self.partial_lines.contains(&self.cursor.pos.y) {
                            adv -= 1;
                        }
                        self.cursor.pos.y += adv;
                    } else {
                        self.cursor.pos.y += 1;
                    }

                    let curr = self.current_line();
                    let len = self.editor.contents[curr].len() - 1;
                    if len == 0 && self.cursor.pos.x > len {
                        self.cursor.pos.x = len;
                        self.cursor.in_line = len;
                    } else if len > 0 && self.cursor.pos.x > len - 1 {
                        self.cursor.pos.x = len - 1;
                        self.cursor.in_line = len - 1;
                    }
                }
            }
            cursor::Direction::Left => {
                let y = self.cursor.pos.y;

                if self.cursor.pos.x != 0 {
                    self.cursor.save_pos();
                    self.cursor.pos.x -= 1;
                    self.cursor.in_line -= 1;
                } else if self.partial_lines.contains(&y) {
                    self.cursor.save_pos();
                    self.cursor.pos.y -= 1;
                    self.cursor.pos.x = self.term.cols() - 1;
                    self.cursor.in_line -= 1;
                }
            }
            cursor::Direction::Right => {
                let (x, y) = self.cursor.current_pos();
                let curr = self.current_line();
                let len = self.editor.contents[curr].len() - 1;

                if len > 0 && self.cursor.in_line < len - 1 {
                    if x < self.term.cols() - 1 {
                        self.cursor.save_pos();
                        self.cursor.pos.x += 1;
                        self.cursor.in_line += 1;
                    } else {
                        self.cursor.save_pos();
                        self.cursor.pos.x = 1;
                        self.cursor.pos.y += 1;
                        self.cursor.in_line += 1;
                    }
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

    /// This function redraws the given line of the UI on the screen.
    pub fn redraw_line(&mut self, ui_line: usize) {
        let file_line = ui_line + self.current_line();
        self.print_line(ui_line, file_line);
    }

    pub fn print_line(&mut self, ui_line: usize, file_line: usize) {
        let contents = &self.editor.contents[file_line];
        let w = self.term.cols();
        let mut i = 0;
        let mut j = 0;

        for &b in contents {
            if b == b'\n' {
                continue;
            } else if i == w {
                i = 0;
                j += 1;
            }
            self.term[(i, ui_line + j)].set_ch(b as char);
            i += 1;
        }

        while i < self.term.cols() {
            self.term[(i, ui_line + j)].set_ch(' ');
            i += 1;
        }
    }
}
