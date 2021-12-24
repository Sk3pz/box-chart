use std::fmt::{Display, Formatter};
use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;

const MAX_BUF_WIDTH: u16 = 200;
const MAX_BUF_HEIGHT: u16 = 60;

pub struct DrawBuf {
    buffer: [[Option<char>; MAX_BUF_WIDTH as usize]; MAX_BUF_HEIGHT as usize],
}

impl DrawBuf {
    pub fn new() -> Self {
        Self {
            buffer: [[None; MAX_BUF_WIDTH as usize]; MAX_BUF_HEIGHT as usize]
        }
    }

    pub fn in_bounds(&self, x: u16, y: u16) -> bool {
        x < self.buffer[0].len() as u16 && y < self.buffer.len() as u16
    }

    pub fn set(&mut self, x: u16, y: u16, c: char) {
        if !self.in_bounds(x, y) {
            panic!("Attempted to write outsize of the buffer!");
        }
        self.buffer[y as usize][x as usize] = Some(c);
    }

    pub fn erase(&mut self, x: u16, y: u16) {
        if !self.in_bounds(x, y) {
            panic!("Attempted to write outsize of the buffer!");
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [[None; MAX_BUF_WIDTH as usize]; MAX_BUF_HEIGHT as usize]
    }

    pub fn peek(&mut self, x: u16, y: u16) -> Option<char> {
        if !self.in_bounds(x, y) {
            panic!("Attempted to write outsize of the buffer!");
        }
        self.buffer[x as usize][y as usize]
    }

    pub fn print<S: Into<String>>(&mut self, mut x: u16, mut y: u16, data: S) {
        if !self.in_bounds(x, y) {
            panic!("Attempted to write outsize of the buffer!");
        }
        let write = data.into().chars().collect::<Vec<_>>();
        for i in 0..write.len() {
            let c = write.get(i).unwrap();
            if x >= MAX_BUF_WIDTH {
                x = 0;
                y += 1;
            }
            self.set(x, y, c.clone());
            x += 1;
        }
    }

    pub fn draw(&self) {
        execute!(stdout(), MoveTo(0,0));
        print!("{}", self);
    }

    fn draw_y(&mut self, x: u16, y: u16, y1: u16) {
        if y == y1 {
            return;
        }
        if y1 < y { // up
            let dy = y - y1;
            self.set(x, y, '┘');
            // the line moves up, so y is lower than y1
            for i in 1..dy {
                self.set(x, y1 + i, '│');
            }
            self.set(x, y1, '┌');
        } else { // down
            let dy = y1 - y;
            self.set(x, y, '┐');
            for i in 1..dy {
                self.set(x, y + i, '│');
            }
            self.set(x, y1, '└');
        }
    }

    pub fn draw_line(&mut self, x: u16, x1: u16, y: u16, y1: u16) {
        // draws a line in the terminal with *'s between two points
        if x > x1 {
            // ensure lines can not go out of bounds
            return;
        }

        let dx = x1 - x;
        let dy_raw = y1 as i16 - y as i16;
        let dy = if dy_raw < 0 {
            (dy_raw * -1) as u16
        } else {
            dy_raw as u16
        };

        // the line is too short for normal calculations
        if dx == 0 {
            if dy != 0 {
                // the line only goes vertical, so no need for change in x
                self.draw_y(x, y, y1);
            } else {
                // the line is only one point
                self.set(x, y, '-');
            }
            return;
        }

        // the line is too short for normal calculations, but too long for the above
        if dx == 1 {
            if dy != 0 {
                // the line only goes vertical, so no need for change in x
                self.draw_y(x, y, y1);
            } else {
                // the line is only one point
                self.print(x, y, "╶╴");
            }
            return;
        }

        // the line is horizontal
        if y == y1 {
            self.print(x, y, format!("╶{}╴", "─".repeat((dx - 1) as usize)));
            return;
        }

        // special edge case for dx == 2
        if dx == 2 {
            if dy == 1 {
                // the line is a single point
                self.print(x, y, "╶╴");
                return;
            }
            // the line is a vertical line
            self.set(x, y, '╶');
            self.draw_y(x + 1, y, y1);
            self.set(x1, y1, '╴');
            return;
        }

        // draw the end points
        self.set(x, y, '╶');
        self.set(x1, y1, '╴');

        // get the length of the two x sides of the line
        let half_x = dx / 2;
        // ensure the line is the correct length in case dx is odd
        // if dx is greater than half_x * 2, then half_x is one less than it should be, so add one to
        // this side to make it even without extending the line beyond what it should be.
        // this adds the difference in dx and half_x * 2 to half_x so if there is a difference, it is
        // added back.
        let right_line_x = half_x + (dx - (half_x * 2));
        // draw the left side of the line
        for i in 0..half_x {
            self.set(x + i + 1, y, '─');
        }
        // draw the vertical portion of the line
        self.draw_y(x + half_x + 1, y, y1);
        // draw the right side of the line
        let right_line_start = x + half_x + 1;
        if right_line_x < 2 {
            // nothing to do here?
            return;
        }
        for i in 0..right_line_x - 2 {
            self.set(right_line_start + i + 1, y1, '─');
        }
    }
}

impl Display for DrawBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let term_size = crossterm::terminal::size().expect("Failed to get terminal size.");
        // todo: algorithm to scale the output buffer to the current terminal size
        //  currently just writes the part of the visible onto the screen
        for line_index in 0..self.buffer.len() {
            if line_index as u16 >= term_size.1 {
                break;
            }
            let line = self.buffer[line_index];
            for char_index in 0..line.len() {
                if char_index as u16 >= term_size.0 {
                    break;
                }
                let c = line[char_index];
                write!(f, "{}", if c.is_some() { c.unwrap() } else { ' ' })?;
            }
            if line_index != self.buffer.len() - 1 && line_index as u16 + 1 < term_size.1 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}