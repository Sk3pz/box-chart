use crate::box_entry::{BoxEntry, BoxPos};

pub mod box_entry;
pub mod node;

fn main() {
    let boxy = BoxEntry::new(
        vec!["Hello, World!", "This is a test of the boxy crate.", "this should test it well."],
        BoxPos::new(50, 10)
    );
    boxy.display();

    let mut boxy2 = BoxEntry::new(
        vec!["Hello, World!", "This is not readable!", "this should test it well."],
        BoxPos::new(20, 10)
    );
    boxy2.display();
    boxy2.close();

    std::thread::sleep(std::time::Duration::from_millis(3000));
    boxy2.open();

    loop {}
}