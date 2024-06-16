use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static STAR_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
pub struct Star<'a> {
    pub rule: Box<dyn Parsable<'a>>,
    pub id: usize,
    pub name: String,
}

impl<'a> Parsable<'a> for Star<'a> {
    fn parse(&self, input: &'a str) -> Option<Node<'a>> {
        let mut node = Node::new_empty(self.id, &self.name);
        let mut size = 0;
        while let Some(child) = self.rule.parse(&input[size..]) {
            size += child.content.len();
            node.add_child(child);
        }
        node.content = &input[0..size];
        Some(node)
    }
    fn get_id(&self) -> &usize {
        &self.id
    }
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[macro_export]
macro_rules! star {
    ($name:expr => $rule:expr) => {
        crate::rule::Star {
            rule: Box::new($rule),
            id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            name: $name.to_string(),
        }
    };
    ($rule:expr) => {
        crate::rule::Star {
            rule: Box::new($rule),
            id: *crate::rule::STAR_ID,
            name: "Star".to_string(),
        }
    };
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn star_rule_matches_zero_times() {
        let rule = star!(char!('a'));
        let input = "";

        let result = rule.parse(input);

        let expected_node = Node::new_empty(rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn star_rule_matches_multiple_times() {
        let rule = star!(char!('a'));
        let input = "aaa";
        let mut expected_node = Node::new("aaa", rule.id, &rule.name);
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));

        let result = rule.parse(input);

        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn star_rule_matches_longer_input() {
        let rule = star!(char!('a'));
        let input = "aaaab";
        let mut expected_node = Node::new("aaaa", *rule.get_id(), rule.get_name());
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));

        let result = rule.parse(input);
        assert_eq!(result, Some(expected_node));
    }
}
