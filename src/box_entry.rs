use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crate::DrawBuf;

/***
 * Unicode stuff
 * TOP_LEFT           ┌
 * TOP_RIGHT          ┐
 * VERTICAL           │
 * HORIZONTAL         ─
 * BOTTOM_LEFT        └
 * BOTTOM_RIGHT       ┘
 * TOP_DOWN_RIGHT     ├
 * TOP_DOWN_LEFT      ┤
 * BOTTOM_UP_RIGHT    ┴
 * BOTTOM_UP_LEFT     ┬
 * RIGHT_UNCONNECTED  ╴
 * LEFT_UNCONNECTED   ╶
 * TOP_UNCONNECTED    ╷
 * BOTTOM_UNCONNECTED ╵
*/

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
    longest: u16,
    closed: bool,
}

impl BoxEntry {

    /// Creates a new box entry from the given lines and position
    pub fn new<S: Into<String>>(lines: Vec<S>, pos: BoxPos) -> Self {
        // todo: line wrapping using MAX_BOX_LINE_LEN
        let mut longest: u16 = 0;
        let mut lns = Vec::new();
        for s in lines {
            let l = s.into();
            if l.len() > longest as usize {
                longest = l.len() as u16;
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
    pub fn inner_size(&self) -> (u16, u16) {
        if self.closed {
            (self.longest, self.lines.len() as u16)
        } else {
            (1, 0)
        }
    }

    /// the size of the box with the border included
    pub fn outer_size(&self) -> (u16, u16) {
        if self.closed {
            (3, 2)
        } else {
            (self.longest + 4, self.lines.len() as u16 + 2)
        }
    }

    /// move the box to a new location
    pub fn move_to(&mut self, x: u16, y: u16, buf: &mut DrawBuf) {
        self.erase();
        self.pos.shift(x, y);
        self.display(buf);
    }

    pub fn erase(&mut self) {
        // clear the box's drawn content
        for x in 0..self.lines.len() + 2 {
            let _ = execute!(stdout(),
            MoveTo(self.pos.x, self.pos.y + x as u16));
            println!("{}", " ".repeat(self.longest as usize + 4));
        }
    }

    pub fn close(&mut self, buf: &mut DrawBuf) {
        self.closed = true;
        self.erase();
        self.display(buf);
    }

    pub fn box_pos(&self) -> BoxPos {
        self.pos
    }

    pub fn open(&mut self, buf: &mut DrawBuf) {
        self.closed = false;
        self.display(buf);
    }

    pub fn toggle(&mut self, buf: &mut DrawBuf) {
        if self.closed {
            self.open(buf);
        } else {
            self.close(buf);
        }
    }

    /// Displays the box at the defined position
    pub fn display(&self, buf: &mut DrawBuf) {
        if self.closed {
            buf.print(self.pos.x, self.pos.y, "+─┐");
            buf.print(self.pos.x, self.pos.y + 1, "└─┘");
            return;
        }
        // print the top border
        buf.print(self.pos.x, self.pos.y,
                  format!("x{}┐", "─".repeat(self.longest as usize + 2)));
        //buf.print(self.pos.x, self.pos.y, format!("┌{}┐", "─".repeat(self.longest + 2))); // for no X in the top left
        // print the text lines
        for y in 0..self.lines.len() {
            let l = self.lines.get(y).unwrap();
            buf.print(self.pos.x, self.pos.y + (y as u16 + 1),
                      format!("│ {}{} │", l, " ".repeat(self.longest as usize - l.len())));
        }
        // print the bottom border
        buf.print(self.pos.x, self.pos.y + self.lines.len() as u16 + 1,
                  format!("└{}┘", "─".repeat(self.longest as usize + 2)));
    }

}