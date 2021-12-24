use std::io::stdout;
use std::time::Duration;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers, poll};
use crossterm::{event, execute};
use crossterm::terminal::{Clear, ClearType, size};
use crate::box_entry::{BoxEntry, BoxPos};
use crate::draw_buffer::DrawBuf;
use crate::node::Node;

pub mod box_entry;
pub mod node;
pub mod draw_buffer;

fn event_loop(buf: &mut DrawBuf) -> crossterm::Result<()> {
    //execute!(stdout(), EnableMouseCapture);
    loop {
        buf.draw();
        // `read()` blocks until an `Event` is available
        match event::read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Char('n') {
                    execute!(stdout(), MoveTo(0, 0));
                    print!("Hi!");
                }
                if event.code == KeyCode::Esc {
                    break;
                }
            },
            Event::Mouse(event) => {

            },
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }
    Ok(())
}

fn main() {
    //execute!(stdout(), Clear(ClearType::All), Hide).unwrap();
    println!("term size: {},{}", size().unwrap().0, size().unwrap().1);

    let mut buf = DrawBuf::new();

    let boxy = BoxEntry::new(
        vec!["Box 1", "this is the left most box", "The first box as well!"],
        BoxPos::new(2, 1)
    );

    let boxy2 = BoxEntry::new(
        vec!["Box 2", "There is a line from", "Box 1 to this!"],
        BoxPos::new(50, 20)
    );

    let node = Node::new(boxy, vec![Node::new(boxy2, vec![])]);
    node.display(&mut buf);

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
    node2.display(&mut buf);

    let mut boxy6 = BoxEntry::new(
        vec!["Box 6", "boxtastic!", "yep, this is also a box."],
        BoxPos::new(48, 5)
    );
    //boxy6.close();

    let mut boxy7 = BoxEntry::new(
        vec!["Box 7", "The last box", "No more boxes after this!"],
        BoxPos::new(79, 7)
    );
    //boxy7.close();

    let node3 = Node::new(boxy6, vec![Node::new(boxy7, vec![])]);
    node3.display(&mut buf);

    let eloop = event_loop(&mut buf);
    execute!(stdout(), Show, MoveTo(0, 100), DisableMouseCapture).unwrap();
    if eloop.is_err() {
        eprintln!("Error occured in the event loop: {}", eloop.unwrap_err());
    }
}