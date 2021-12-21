use std::io::{stdout, Write};
use crossterm::cursor::MoveTo;
use crate::BoxEntry;
use crate::line::draw_line;

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

    pub fn display(&self) {
        let start_x = self.boxy.box_pos().x + self.boxy.outer_size().0;
        let start_y = self.boxy.box_pos().y + self.boxy.outer_size().1 / 2;
        for b in &self.branches {
            let end_x = b.boxy.box_pos().x - 1;
            let end_y = b.boxy.box_pos().y + b.boxy.outer_size().1 / 2;
            draw_line(start_x, end_x, start_y, end_y);
            b.display();
        }
        self.boxy.display();
    }
}