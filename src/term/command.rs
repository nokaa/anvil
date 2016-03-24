/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

use super::{Term};
use editor::EditorMode;

/// Handles command `ch` for command mode
pub fn handle(term: &mut Term, ch: char) {
    match ch {
        '\x1b' => { // Escape key
            term.editor.switch_mode(EditorMode::Normal);
        }
        'w' => {
            // TODO(nokaa): writing to file should display success/failure
            // to user and not exit terminal. We can show this status
            // in the line below the prompt.
            match term.editor.write_file() {
                Ok(_) => { }
                Err(e) => {
                    term.quit();
                    println!("{}", e);
                }
            }
        }
        'q' => {
            term.quit();
        }
        _ => { }
    }
}
