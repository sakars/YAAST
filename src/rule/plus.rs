use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static PLUS_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Plus<'a> {
    pub rule: Box<dyn Parsable<'a>>,
    pub id: usize,
    pub name: String,
}

impl<'a> Parsable<'a> for Plus<'a> {
    fn parse(&self, input: &'a str) -> Option<Node<'a>> {
        let mut node = Node::new_empty(self.id, &self.name);
        let mut size = 0;
        if let Some(child) = self.rule.parse(input) {
            size += child.content.len();
            node.add_child(child);
        } else {
            return None;
        }
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
macro_rules! plus {
    ($name:expr => $rule:expr) => {
        crate::rule::Plus {
            rule: Box::new($rule),
            id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            name: $name.to_string(),
        }
    };
    ($rule:expr) => {
        crate::rule::Plus {
            rule: Box::new($rule),
            id: *crate::rule::PLUS_ID,
            name: "Plus".to_string(),
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn plus_rule_matches_one_time() {
        let rule = plus!(char!('a'));
        let input = "a";

        let result = rule.parse(input);

        let mut expected_node = Node::new("a", *rule.get_id(), rule.get_name());
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn plus_rule_matches_multiple_times() {
        let rule = plus!(char!('a'));
        let input = "aaa";
        let mut expected_node = Node::new("aaa", *rule.get_id(), rule.get_name());
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));

        let result = rule.parse(input);

        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn plus_rule_does_not_match_zero_times() {
        let rule = plus!(char!('a'));
        let input = "";

        let result = rule.parse(input);

        assert_eq!(result, None);
    }

    #[test]
    fn plus_rule_matches_longer_input() {
        let rule = plus!(char!('a'));
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
