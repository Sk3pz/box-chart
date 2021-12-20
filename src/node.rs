use std::io::{stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crate::BoxEntry;

#[derive(Clone, Debug)]
pub struct Node {
    boxy: BoxEntry,
    branches: Vec<Node>,
}

impl Node {
    pub fn new(boxy: BoxEntry, branches: Vec<Node>) -> Node {
        Node {
            boxy,
            branches,
        }
    }

    // todo(eric): known issues:
    //  - if the points are too close, it crashes
    //  - if the second point is behind the first (on the x axis), it crashes
    //  - sometimes the y axis part of the line is shifted by 1
    fn draw_line(&self, x: u16, x1: u16, y: u16, y1: u16) {
        // draws a line in the terminal with *'s between two points
        let mut stdout = stdout();
        execute!(stdout, MoveTo(x, y)).unwrap();
        write!(stdout, "╶").unwrap();
        execute!(stdout, MoveTo(x1, y1)).unwrap();
        write!(stdout, "╴").unwrap();

        let dx = x1 - x;
        if y == y1 {
            execute!(stdout, MoveTo(x + 1, y)).unwrap();
            for _ in 1..dx {
                print!("─");
            }
            return;
        }
        let mut up = false;
        let dy_raw = y1 as i16 - y as i16;
        let mut dy = if dy_raw < 0 {
            up = true;
            (dy_raw * -1) as u16
        } else {
            dy_raw as u16
        };
        let half_x = dx / 2;
        let mut right_line_x = half_x;
        if half_x * 2 < dx {
            right_line_x += 1;
        }
        for i in 0..half_x {
            execute!(stdout, MoveTo(x + i + 1, y)).unwrap();
            print!("─");
        }
        execute!(stdout, MoveTo(x + half_x + 1, y)).unwrap();
        if up {
            print!("┘");
            // the line moves up, so y is lower than y1
            for i in 1..dy {
                execute!(stdout, MoveTo(x1 - half_x + 1, y1 + i)).unwrap();
                print!("│");
            }
            execute!(stdout, MoveTo(x1 - half_x + 1, y1)).unwrap();
            print!("┌");
        } else {
            print!("┐");
            for i in 1..dy {
                execute!(stdout, MoveTo(x + half_x + 1, y + i)).unwrap();
                print!("│");
            }
            execute!(stdout, MoveTo(x + half_x + 1, y1)).unwrap();
            print!("└");
        }
        let right_line_start = x + half_x + 1;
        for i in 0..right_line_x - 2 {
            execute!(stdout, MoveTo(right_line_start + i + 1, y1)).unwrap();
            print!("─");
        }
    }

    pub fn display(&self) {
        let start_x = self.boxy.box_pos().x + self.boxy.outer_size().0;
        let start_y = self.boxy.box_pos().y + self.boxy.outer_size().1 / 2;
        for b in &self.branches {
            let end_x = b.boxy.box_pos().x - 1;
            let end_y = b.boxy.box_pos().y + b.boxy.outer_size().1 / 2;
            self.draw_line(start_x, end_x, start_y, end_y);
            b.display();
        }
        self.boxy.display();
    }
}