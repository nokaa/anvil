use file;

pub struct Editor {
    mode: EditorMode,
    filename: String,
    contents: Vec<u8>,
}

#[derive(PartialEq)]
pub enum EditorMode {
    Command,
    Insert,
}

impl Editor {
    pub fn new(filename: String) -> Editor {
        let contents: Vec<u8>;
        if filename == "".to_string() {
            contents = vec![b'F', b'o', b'r', b'g', b'e'];
        } else {
            contents = file::read_file(&filename[..]);
        }

        Editor {
            mode: EditorMode::Command,
            filename: filename,
            contents: contents,
        }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
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
