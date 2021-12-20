use std::io::stdout;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use crate::box_entry::{BoxEntry, BoxPos};
use crate::node::Node;

pub mod box_entry;
pub mod node;

fn main() {
    execute!(stdout(), Clear(ClearType::All), Hide);
    let boxy = BoxEntry::new(
        vec!["Box 1", "this is the left most box", "The first box as well!"],
        BoxPos::new(2, 5)
    );

    let boxy2 = BoxEntry::new(
        vec!["Box 2", "There is a line from", "Box 1 to this!"],
        BoxPos::new(50, 20)
    );

    let node = Node::new(boxy, vec![Node::new(boxy2, vec![])]);
    node.display();

    let boxy3 = BoxEntry::new(
        vec!["Box 3", "Boxy!", "Small Box"],
        BoxPos::new(48, 15)
    );

    let boxy4 = BoxEntry::new(
        vec!["Box 4", "Idk what to type here", "Some Cool things"],
        BoxPos::new(90, 10)
    );

    let boxy5 = BoxEntry::new(
        vec!["Box 5", "short.", "short.", "tall.", "tall."],
        BoxPos::new(124, 15)
    );

    let node2 = Node::new(boxy3, vec![Node::new(boxy4,
                                                vec![Node::new(boxy5, vec![])])]);
    node2.display();

    let mut boxy6 = BoxEntry::new(
        vec!["Box 6", "boxtastic!", "yep, this is also a box."],
        BoxPos::new(48, 5)
    );
    //boxy6.close();

    let mut boxy7 = BoxEntry::new(
        vec!["Box 7", "The last box", "No more boxes after this!"],
        BoxPos::new(90, 5)
    );
    //boxy7.close();

    let node3 = Node::new(boxy6, vec![Node::new(boxy7, vec![])]);
    node3.display();

    execute!(stdout(), Show, MoveTo(0, 100));
    loop {

    }
}