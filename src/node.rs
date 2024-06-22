use std::{sync::atomic::AtomicUsize, vec};

use once_cell::sync::Lazy;

use layout::{self, backends::svg::SVGWriter};

pub static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)] // Add the Debug trait
pub struct Node<'a> {
    pub type_id: usize,
    pub type_name: String,
    pub content: &'a str,
    pub children: Vec<Node<'a>>,
}

pub static ROOT_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub static UNREACHABLE_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

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

    pub fn new_as_root(node: Node<'a>) -> Node<'a> {
        Node {
            content: node.content,
            children: vec![node],
            type_id: *ROOT_ID,
            type_name: "Root".to_string(),
        }
    }

    pub fn new_as_unreachable() -> Node<'a> {
        Node {
            content: "",
            children: Vec::new(),
            type_id: *UNREACHABLE_ID,
            type_name: "Unreachable".to_string(),
        }
    }

    pub fn add_child(&mut self, child: Node<'a>) {
        self.children.push(child);
    }

    pub fn children(&self) -> &Vec<Node<'a>> {
        &self.children
    }

    fn to_dot_private(&self, id: &mut usize) -> String {
        let content_preview = if self.content.len() > 10 {
            format!("{}...", &self.content[..10])
        } else {
            self.content.to_string()
        };

        let mut result = format!(
            "{} [label=\"{}\\n\\\"{}\\\"\"];\n",
            id, self.type_name, content_preview
        );

        let my_id = *id;
        *id += 1;
        for child in &self.children {
            result += &format!("{} -> {};\n", my_id, *id);
            result += &child.to_dot_private(id);
        }
        result
    }

    pub fn to_dot(&self) -> String {
        let mut id = 0;
        let mut result = "digraph G {\n".to_string();
        result += &self.to_dot_private(&mut id);
        result += "}\n";
        result
    }

    pub fn make_svg(&self, filename: &str) {
        let mut svg = SVGWriter::new();
        let dot = self.to_dot();
        let mut parser = layout::gv::DotParser::new(&dot);
        let graph = parser.process().unwrap();
        let mut builder = layout::gv::GraphBuilder::new();
        builder.visit_graph(&graph);
        let mut visual_graph = builder.get();
        visual_graph.do_it(false, false, false, &mut svg);
        let s = svg.finalize();
        std::fs::write(filename, s).unwrap();
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
