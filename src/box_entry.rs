use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;

const _RIGHT_UNCONNECTED: &str = "╴";
const _LEFT_UNCONNECTED:  &str = "╶";

const _MAX_BOX_LINE_LEN: usize = 30;

/// Stores the position of a box
#[derive(Clone, Copy, Debug)]
pub struct BoxPos {
    pub x: u16,
    pub y: u16
}

impl BoxPos {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn shift(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}

/// A box entry for display
#[derive(Clone, Debug)]
pub struct BoxEntry {
    lines: Vec<String>,
    pos: BoxPos,
    // todo(eric): have something to store linked boxes for connecting lines
    longest: usize,
    closed: bool,
}

impl BoxEntry {

    /// Creates a new box entry from the given lines and position
    pub fn new<S: Into<String>>(lines: Vec<S>, pos: BoxPos) -> Self {
        // todo: line wrapping using MAX_BOX_LINE_LEN
        let mut longest = 0;
        let mut lns = Vec::new();
        for s in lines {
            let l = s.into();
            if l.len() > longest {
                longest = l.len();
            }
            lns.push(l);
        }
        Self {
            lines: lns,
            pos,
            longest,
            closed: false
        }
    }

    /// the size of the box without the border
    pub fn inner_size(&self) -> (usize, usize) {
        (self.longest, self.lines.len())
    }

    /// the size of the box with the border included
    pub fn outer_size(&self) -> (usize, usize) {
        (self.longest + 2, self.lines.len()+ 2)
    }

    /// move the box to a new location
    pub fn move_to(&mut self, x: u16, y: u16) {
        self.pos.shift(x, y);
    }

    /// Displays the box at the defined position
    pub fn display(&self) {
        if self.closed {
            let _ = execute!(stdout(),
            MoveTo(self.pos.x, self.pos.y));
            println!("+");
        }
        // print the top border
        let _ = execute!(stdout(),
        MoveTo(self.pos.x, self.pos.y));
        //println!("┌{}┐", "─".repeat(self.longest + 2));
        println!("┌{}x", "─".repeat(self.longest + 2));
        // print the text lines
        for y in 0..self.lines.len() {
            let l = self.lines.get(y).unwrap();
            let _ = execute!(stdout(),
            MoveTo(self.pos.x, self.pos.y + (y as u16 + 1)));
            println!("│ {}{} │", l, " ".repeat(self.longest - l.len()));
        }
        // print the bottom border
        let _ = execute!(stdout(),
        MoveTo(self.pos.x, self.pos.y + self.lines.len() as u16 + 1));
        println!("└{}┘", "─".repeat(self.longest + 2));
    }

}