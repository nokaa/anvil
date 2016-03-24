/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

use rustty;

/// Represents a direction for movement in the UI
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Represents an `x, y` coordinate.
#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// Represents the cursor in our terminal
pub struct Cursor {
    pub pos: Position,
    pub lpos: Position,
    pub color: rustty::Color,
}

impl Cursor {
    /// Creates a new cursor
    pub fn new(color: rustty::Color) -> Cursor {
        Cursor {
            pos: Position {x: 0, y: 0},
            lpos: Position {x: 0, y: 0},
            color: color,
        }
    }

    /// Returns a pair of the current `(x, y)` position
    pub fn current_pos(&self) -> (usize, usize) {
        (self.pos.x, self.pos.y)
    }

    /// Returns a pair of the last `(x, y)` position
    pub fn last_pos(&self) -> (usize, usize) {
        (self.lpos.x, self.lpos.y)
    }

    /// Stores the current position as the last position
    pub fn save_pos(&mut self) {
        self.lpos = self.pos;
    }
}
