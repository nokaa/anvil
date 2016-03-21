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
            term.cursor.save_pos();
            if term.cursor.pos.x == 0 {
                term.cursor.pos.y = term.cursor.pos.y.saturating_sub(1);

                let curr = term.cursor.pos.y + term.current_line();
                let len = term.editor.contents[curr].len() - 1;
                term.cursor.pos.x = len;
            } else {
                term.cursor.pos.x -= 1;
            }
            term.term[term.cursor.current_pos()].set_ch(' ');
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
            term.term[term.cursor.current_pos()].set_ch(c);
            term.cursor.save_pos();
            term.cursor.pos.x += 1;
        }
    }
}
