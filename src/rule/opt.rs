use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static OPT_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Opt<'a> {
    pub rule: Box<dyn Parsable<'a>>,
    pub id: usize,
    pub name: String,
}

impl<'a> Parsable<'a> for Opt<'a> {
    fn parse(&self, input: &'a str) -> Option<Node<'a>> {
        if let Some(node) = self.rule.parse(input) {
            Some(node)
        } else {
            Some(Node::new_empty(self.id, &self.name))
        }
    }
    fn get_id(&self) -> &usize {
        &self.id
    }
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[macro_export]
macro_rules! opt {
    ($name:expr => $rule:expr) => {
        crate::rule::Opt {
            rule: Box::new($rule),
            id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            name: $name.to_string(),
        }
    };
    ($rule:expr) => {
        crate::rule::Opt {
            rule: Box::new($rule),
            id: *crate::rule::OPT_ID,
            name: "Opt".to_string(),
        }
    };
}
