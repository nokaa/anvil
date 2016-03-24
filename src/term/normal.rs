/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

use super::{Term, cursor};
use editor::EditorMode;
use rustty::{Event};

pub fn handle(term: &mut Term, ch: char) {
    match ch {
        'i' => {
            term.editor.switch_mode(EditorMode::Insert);
        }
        ':' => {
            term.editor.switch_mode(EditorMode::Command);
        }
        'h' => {
            term.move_cursor(cursor::Direction::Left);
        }
        'j' => {
            term.move_cursor(cursor::Direction::Down);
        }
        'k' => {
            term.move_cursor(cursor::Direction::Up);
        }
        'l' => {
            term.move_cursor(cursor::Direction::Right);
        }
        'r' => {
            let evt = term.term.get_event(-1).unwrap();
            if let Some(Event::Key(ch)) = evt {
                match ch {
                    '\x1b' | '\x7f' => { }
                    // TODO(nokaa): ENTER and TAB are ignored for now.
                    '\r' | '\t' => { }
                    c @ _ => {
                        let pos = term.cursor.current_pos();
                        term.term[pos].set_ch(c);
                        term.editor.replace_char(pos, c);
                    }
                }
            }
        }
        'x' => {
            let (x, y) = term.cursor.current_pos();
            term.editor.contents[y].remove(x);
            term.redraw_line(y);
        }
        _ => { }
    }
}
