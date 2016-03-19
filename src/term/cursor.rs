use rustty;

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
    pub fn new(color: rustty::Color) -> Cursor {
        Cursor {
            pos: Position {x: 0, y: 0},
            lpos: Position {x: 0, y: 0},
            color: color,
        }
    }
}
