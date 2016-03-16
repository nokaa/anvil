/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

use file;

/// A struct representing an editor
pub struct Editor {
    /// Represents the mode the editor
    /// is currently in
    mode: EditorMode,
    /// Represents the name of the file
    /// we are working with
    filename: String,
    /// Represents the contents of the file
    contents: Vec<u8>,
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

impl Editor {
    /// Creates a new editor. If filename is the empty
    /// string, the contents are `Forge`. Otherwise, the
    /// given filename is read as the contents.
    pub fn new(filename: String) -> Editor {
        let contents: Vec<u8>;
        if filename == "".to_string() {
            contents = vec![b'F', b'o', b'r', b'g', b'e'];
        } else {
            if file::file_exists(&filename[..]) {
                // It *should* be safe to unwrap here, since we have already checked
                // that `filename` exists.
                contents = file::read_file(&filename[..]).unwrap();
            } else {
                contents = vec![];
            }
        }

        Editor {
            mode: EditorMode::Command,
            filename: filename,
            contents: contents,
        }
    }

    /// Returns the name of the file we are working with.
    pub fn filename(&self) -> String {
        self.filename.clone()
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
}
