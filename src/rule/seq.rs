use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static SEQ_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Seq<'a> {
    pub rules: Vec<crate::rule::Rule<'a>>,
}

impl<'a> Parsable<'a> for Seq<'a> {
    fn parse(&self, input: &'a str, id: usize, name: &String) -> Option<Node<'a>> {
        let mut node = Node::new_empty(id, name);
        let mut size = 0;
        for rule in &self.rules {
            if let Some(child) = rule.parse(&input[size..]) {
                size += child.content.len();
                node.add_child(child);
            } else {
                return None;
            }
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
        for rule in &self.rules {
            if let Some(child) = rule.parse_with_handler(&input[size..], handler) {
                size += child.content.len();
                node.add_child(child);
            } else {
                handler.handle_failure(id);
                return None;
            }
        }
        node.content = &input[0..size];
        handler.handle_success(&mut node);
        Some(node)
    }
}

#[macro_export]
macro_rules! seq {
    ($($rule:expr),*) => {
        $crate::rule::Rule::new(
            Box::new(crate::rule::Seq {
                rules: vec![$($rule.clone()),*],
            }),
            *crate::rule::SEQ_ID,
            "Seq".to_string(),
        )
    };
    ($name:expr => $($rule:expr),*) => {
        $crate::custom!($name => $crate::seq!($($rule),*))
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rule_handler::Handler;

    #[test]
    fn seq_rule_matches_multiple_rules() {
        let rule = seq!(char!('a'), char!('b'));
        let input = "ab";
        let mut expected_node = Node::new("ab", rule.id, &rule.name);
        expected_node.add_child(Node::new("a", rule.id, &rule.name));
        expected_node.add_child(Node::new("b", rule.id, &rule.name));

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn seq_rule_does_not_match_partial_input() {
        let rule = seq!(char!('a'), char!('b'));
        let input = "a";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, None);
    }

    #[test]
    fn seq_rule_does_not_match_empty_input() {
        let rule = seq!(char!('a'), char!('b'));
        let input = "";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, None);
    }

    #[test]
    fn seq_rule_does_match_longer_input() {
        let rule = seq!(char!('a'), char!('b'));
        let input = "abc";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        let mut expected_node = Node::new("ab", rule.id, &rule.name);
        expected_node.add_child(Node::new("a", rule.id, &rule.name));
        expected_node.add_child(Node::new("b", rule.id, &rule.name));
        assert_eq!(result, Some(expected_node));
    }
}
