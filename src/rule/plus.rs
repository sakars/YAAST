use crate::COUNTER;

use super::Rule;
use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static PLUS_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Plus<'a> {
    pub rule: Rule<'a>,
}

impl<'a> Parsable<'a> for Plus<'a> {
    fn parse(&self, input: &'a str, id: usize, name: &String) -> Option<Node<'a>> {
        let mut node = Node::new_empty(id, &name);
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

    fn parse_with_handler(
        &self,
        input: &'a str,
        id: usize,
        name: &String,
        handler: &crate::rule_handler::Handler<'a>,
    ) -> Option<Node<'a>> {
        handler.handle_pre_parse(id);
        let mut node = Node::new_empty(id, name);
        let mut size = 0;
        if let Some(child) = self.rule.parse_with_handler(input, handler) {
            size += child.content.len();
            node.add_child(child);
        } else {
            handler.handle_failure(id);
            return None;
        }
        while let Some(child) = self.rule.parse_with_handler(&input[size..], handler) {
            size += child.content.len();
            node.add_child(child);
        }
        node.content = &input[0..size];
        handler.handle_success(&mut node);
        Some(node)
    }
}

#[macro_export]
macro_rules! plus {
    ($name:expr => $rule:expr) => {
        $crate::custom!($name => $crate::plus!($rule))
    };
    ($rule:expr) => {
        $crate::rule::Rule::new(
            Box::new($crate::rule::Plus { rule: $rule }),
            *$crate::rule::PLUS_ID,
            "Plus".to_string(),
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rule_handler::Handler;

    #[test]
    fn plus_rule_matches_one_time() {
        let rule = plus!(char!('a'));
        let input = "a";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        let mut expected_node = Node::new("a", rule.id, &rule.name);
        expected_node.add_child(Node::new("a", rule.id, &rule.name));
        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn plus_rule_matches_multiple_times() {
        let rule = plus!(char!('a'));
        let input = "aaa";
        let mut expected_node = Node::new("aaa", rule.id, &rule.name);
        expected_node.add_child(Node::new("a", rule.id, &rule.name));
        expected_node.add_child(Node::new("a", rule.id, &rule.name));
        expected_node.add_child(Node::new("a", rule.id, &rule.name));

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn plus_rule_does_not_match_zero_times() {
        let rule = plus!(char!('a'));
        let input = "";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, None);
    }

    #[test]
    fn plus_rule_matches_longer_input() {
        let rule = plus!(char!('a'));
        let input = "aaaab";
        let mut expected_node = Node::new("aaaa", rule.id, &rule.name);
        expected_node.add_child(Node::new("a", rule.id, &rule.name));
        expected_node.add_child(Node::new("a", rule.id, &rule.name));
        expected_node.add_child(Node::new("a", rule.id, &rule.name));
        expected_node.add_child(Node::new("a", rule.id, &rule.name));

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);
        assert_eq!(result, Some(expected_node));
    }
}
