/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

use super::{Term, cursor};

pub fn handle(term: &mut Term, ch: char) {
    match ch {
        'i' => {
            term.editor.switch_mode();
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
        'q' => {
            term.quit();
        }
        _ => { }
    }
}
