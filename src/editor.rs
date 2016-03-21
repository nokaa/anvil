/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

use file;

/// A struct representing an editor
pub struct Editor<'a> {
    /// Represents the mode the editor
    /// is currently in
    mode: EditorMode,
    /// Represents the name of the file
    /// we are working with
    filename: &'a str,
    /// Represents the contents of the file
    pub contents: Vec<Vec<u8>>,
}

/// This enum represents what mode the editor is in.
/// Modes are analogous to the concept found in the
/// Vi editor.
#[derive(PartialEq)]
pub enum EditorMode {
    /// Command mode
    Command,
    /// Insert mode
    Insert,
}

impl<'a> Editor<'a> {
    /// Creates a new editor. If filename is the empty
    /// string, the contents are `Forge`. Otherwise, the
    /// given filename is read as the contents.
    pub fn new(filename: &str) -> Editor {
        let contents: Vec<Vec<u8>> = vec![vec![]];

        Editor {
            mode: EditorMode::Command,
            filename: filename,
            contents: contents,
        }
    }

    /// Reads `self.filename` to a vector of lines. A line is determined
    /// by a newline character, or when a line's length is the same as
    /// the given `line_length`. `\n` is included in the line, and must
    /// be ignored when printing in the UI.
    pub fn read_file(&mut self, line_length: usize) {
        let filename = self.filename;
        if filename == "" {
            self.contents = vec![vec![b'F', b'o', b'r', b'g', b'e']];
        } else if file::file_exists(filename) {
            // It *should* be safe to unwrap here, since we have already
            // checked that `filename` exists.
            self.contents = file::read_file_lines(filename, line_length)
                .unwrap();
        }
    }

    /// Returns the name of the file we are working with.
    pub fn filename(&self) -> &str {
        self.filename
    }

    /// Returns true if we are in `Command` mode,
    /// false otherwise.
    pub fn command_mode(&self) -> bool {
        self.mode == EditorMode::Command
    }

    /// Switches the mode that the editor is in.
    pub fn switch_mode(&mut self) {
        use self::EditorMode::*;

        match self.mode {
            Command => self.mode = Insert,
            Insert => self.mode = Command,
        }
    }

    pub fn replace_char(&mut self, (x, y): (usize, usize), c: char) {
        // contents[line][position in line]
        self.contents[y][x] = c as u8;
    }
}
