pub struct Editor {
    mode: EditorMode,
    filename: String,
}

#[derive(PartialEq)]
pub enum EditorMode {
    Command,
    Insert,
}

impl Editor {
    pub fn new(filename: String) -> Editor {
        Editor {
            mode: EditorMode::Command,
            filename: filename,
        }
    }

    pub fn command_mode(&self) -> bool {
        self.mode == EditorMode::Command
    }

    pub fn switch_mode(&mut self) {
        use self::EditorMode::*;

        match self.mode {
            Command => self.mode = Insert,
            Insert => self.mode = Command,
        }
    }
}
