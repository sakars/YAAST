use std::sync::atomic::AtomicUsize;

pub static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)] // Add the Debug trait
pub struct Node<'a> {
    pub content: &'a str,
    pub children: Vec<Node<'a>>,
    pub type_id: usize,
    pub type_name: String,
}

pub trait Parsable<'a> {
    fn parse(&self, input: &'a str, id: usize, name: &String) -> Option<Node<'a>>;
}

impl<'a> Node<'a> {
    pub fn new(content: &'a str, id: usize, name: &String) -> Node<'a> {
        Node {
            content,
            children: Vec::new(),
            type_id: id,
            type_name: name.clone(),
        }
    }

    pub fn new_empty(id: usize, name: &String) -> Node<'a> {
        Node {
            content: "",
            children: Vec::new(),
            type_id: id,
            type_name: name.clone(),
        }
    }

    pub fn add_child(&mut self, child: Node<'a>) {
        self.children.push(child);
    }

    pub fn children(&self) -> &Vec<Node<'a>> {
        &self.children
    }
}

impl std::cmp::PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.children.len() != other.children.len() {
            return false;
        }
        for (a, b) in self.children.iter().zip(other.children.iter()) {
            if a != b {
                return false;
            }
        }
        self.content == other.content && self.children == other.children
    }
}
