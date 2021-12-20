use crate::box_entry::{BoxEntry, BoxPos};

pub mod box_entry;

fn main() {
    println!("┌───────────────x\n\
              │ Hello, world! │\n\
              │ Testing!      │\n\
              └───────────────┘");

    let boxy = BoxEntry::new(
        vec!["Hello, World!", "Testing!"],
        BoxPos::new(50, 10)
    );
    boxy.display();
}
