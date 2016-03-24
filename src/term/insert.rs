/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

use super::Term;
use editor::EditorMode;

pub fn handle(term: &mut Term, ch: char) {
    match ch {
        '\x1b' => { // Escape key
            term.cursor.save_pos();
            if term.cursor.pos.x > 0 {
                term.cursor.pos.x -= 1;
            }

            term.editor.switch_mode(EditorMode::Normal);
        }
        '\x7f' => { // Backspace key
            let line = term.current_line() + term.cursor.pos.y;
            term.cursor.save_pos();

            if term.cursor.pos.x == 0 {
                let mut rem = term.editor.contents[line].clone();
                let pos = term.editor.contents[line - 1].len() - 1;

                term.editor.contents[line - 1].remove(pos);
                term.editor.contents[line - 1].append(&mut rem);
                term.editor.contents.remove(line);

                if term.cursor.pos.y != 0 {
                    term.cursor.pos.y -= 1;
                }

                term.cursor.pos.x = pos;
                term.redraw_file();
            } else {
                term.cursor.pos.x -= 1;
                let pos = term.cursor.pos.x;

                term.editor.contents[line].remove(pos);

                let y = term.cursor.pos.y;
                term.redraw_line(y);
            }
        }
        '\r' => {
            let line = term.current_line() + term.cursor.pos.y;
            let pos = term.cursor.pos.x;

            let mut rem = term.editor.contents[line].split_off(pos);
            rem.push(b'\n');
            term.editor.contents.insert(line + 1, rem);

            term.cursor.save_pos();
            term.cursor.pos.x = 0;
            term.cursor.pos.y += 1;

            term.redraw_file();
        }
        '\t' => {
            let line = term.current_line() + term.cursor.pos.y;
            let pos = term.cursor.pos.x;

            for _ in 0..4 {
                term.editor.contents[line].insert(pos, b' ');
            }

            term.cursor.save_pos();
            term.cursor.pos.x += 4;

            let y = term.cursor.pos.y;
            term.redraw_line(y);
        }
        c @ _ => {
            let line = term.current_line() + term.cursor.pos.y;
            let pos = term.cursor.pos.x;

            term.editor.contents[line].insert(pos, c as u8);

            term.cursor.save_pos();
            term.cursor.pos.x += 1;

            let y = term.cursor.pos.y;
            term.redraw_line(y);
        }
    }
}
