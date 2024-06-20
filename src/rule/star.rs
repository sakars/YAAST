use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

use super::Rule;

pub static STAR_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
pub struct Star<'a> {
    pub rule: Rule<'a>,
}

impl<'a> Parsable<'a> for Star<'a> {
    fn parse(&self, input: &'a str, id: usize, name: &String) -> Option<Node<'a>> {
        let mut node = Node::new_empty(id, &name);
        let mut size = 0;
        while let Some(child) = self.rule.parse(&input[size..]) {
            size += child.content.len();
            node.add_child(child);
        }
        node.content = &input[0..size];
        Some(node)
    }
}

#[macro_export]
macro_rules! star {
    ($name:expr => $rule:expr) => {
        $crate::custom!($name => $crate::star!($rule))
    };
    ($rule:expr) => {
        $crate::rule::Rule::new(
            Box::new($crate::rule::Star { rule: $rule }),
            *$crate::rule::STAR_ID,
            "Star".to_string(),
        )
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
        expected_node.add_child(Node::new("a", rule.id, &rule.name));
        expected_node.add_child(Node::new("a", rule.id, &rule.name));
        expected_node.add_child(Node::new("a", rule.id, &rule.name));

        let result = rule.parse(input);

        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn star_rule_matches_longer_input() {
        let rule = star!(char!('a'));
        let input = "aaaab";
        let mut expected_node = Node::new("aaaa", rule.id, &rule.name);
        expected_node.add_child(Node::new("a", rule.id, &"Char".to_string()));
        expected_node.add_child(Node::new("a", rule.id, &"Char".to_string()));
        expected_node.add_child(Node::new("a", rule.id, &"Char".to_string()));
        expected_node.add_child(Node::new("a", rule.id, &"Char".to_string()));

        let result = rule.parse(input);
        assert_eq!(result, Some(expected_node));
    }
}
