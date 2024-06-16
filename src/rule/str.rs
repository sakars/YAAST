use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static STR_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Str {
    pub s: String,
    pub id: usize,
    pub name: String,
}

impl<'a> Parsable<'a> for Str {
    fn parse(&self, input: &'a str) -> Option<Node<'a>> {
        if input.starts_with(&self.s) {
            Some(Node::new(&input[0..self.s.len()], self.id, &self.name))
        } else {
            None
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
macro_rules! str {
    ($s:expr) => {
        crate::rule::Str {
            s: $s.to_string(),
            id: *crate::rule::STR_ID,
            name: "Str".to_string(),
        }
    };
    ($name:expr => $s:expr) => {
        crate::rule::Str {
            s: $s.to_string(),
            id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            name: $name.to_string(),
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn str_rule_matches_string() {
        let rule = str!("hello");
        let input = "hello";
        let expected_node = Node::new("hello", rule.id, &rule.name);

        let result = rule.parse(input);

        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn str_rule_does_not_match_different_string() {
        let rule = str!("hello");
        let input = "world";

        let result = rule.parse(input);

        assert_eq!(result, None);
    }

    #[test]
    fn str_rule_does_not_match_empty_input() {
        let rule = str!("hello");
        let input = "";

        let result = rule.parse(input);

        assert_eq!(result, None);
    }

    #[test]
    fn str_rule_does_not_match_shorter_input() {
        let rule = str!("hello");
        let input = "hell";

        let result = rule.parse(input);

        assert_eq!(result, None);
    }

    #[test]
    fn str_rule_does_match_longer_input() {
        let rule = str!("hello");
        let input = "hello world";

        let result = rule.parse(input);
        let expected_node = Node::new("hello", rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }
}
