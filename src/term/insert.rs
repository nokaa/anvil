/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

use super::Term;

pub fn handle(term: &mut Term, ch: char) {
    match ch {
        '\x1b' => { // Escape key
            term.editor.switch_mode();
        }
        '\x7f' => { // Backspace key
            let line = term.current_line() + term.cursor.pos.y;
            term.cursor.save_pos();
            if term.cursor.pos.x == 0 {
                term.editor.contents.remove(line);
                if term.cursor.pos.y != 0 {
                    term.cursor.pos.y -= 1;
                }

                let line = term.current_line() + term.cursor.pos.y;
                let len = term.editor.contents[line].len() - 1;
                term.cursor.pos.x = len;
                term.redraw_file();
            } else {
                term.cursor.pos.x -= 1;
                let pos = term.cursor.pos.x;
                term.editor.contents[line].remove(pos);
                term.redraw_file();
            }
        }
        '\r' => {
            let line = term.current_line() + term.cursor.pos.y;
            term.editor.contents.insert(line, vec![b'\n']);
            term.cursor.save_pos();
            term.cursor.pos.x = 0;
            term.cursor.pos.y += 1;
            term.redraw_file();
        }
        '\t' => {
            for i in 0..4 {
                let x = term.cursor.pos.x + i;
                term.term[(x, term.cursor.pos.y)].set_ch(' ');
            }
            term.cursor.save_pos();
            term.cursor.pos.x += 4;
        }
        c @ _ => {
            /*let line = term.current_line() + term.cursor.pos.y;
            let pos = term.cursor.pos.x;
            term.editor.contents[line, pos*/
            term.term[term.cursor.current_pos()].set_ch(c);
            term.cursor.save_pos();
            term.cursor.pos.x += 1;
        }
    }
}
