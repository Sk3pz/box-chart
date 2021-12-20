use crate::BoxEntry;

#[derive(Clone, Debug)]
pub struct Node {
    branches: Vec<Node>,
    boxy: BoxEntry,
}