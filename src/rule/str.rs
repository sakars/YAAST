use crate::COUNTER;

use super::*;
use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static STR_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Str {
    pub s: String,
}

impl<'a> Parsable<'a> for Str {
    fn parse(&self, input: &'a str, id: usize, name: &String) -> Option<Node<'a>> {
        if input.starts_with(&self.s) {
            Some(Node::new(&input[0..self.s.len()], id, &name))
        } else {
            None
        }
    }

    fn parse_with_handler(
        &self,
        input: &'a str,
        id: usize,
        name: &String,
        handler: &Handler<'a>,
    ) -> Option<Node<'a>> {
        handler.handle_pre_parse(id);
        if let Some(mut success) = self.parse(input, id, name) {
            handler.handle_success(&mut success);
            Some(success)
        } else {
            handler.handle_failure(id);
            None
        }
    }
}

#[macro_export]
macro_rules! str {
    ($s:expr) => {
        $crate::rule::Rule::new(
            Box::new($crate::rule::Str { s: $s.to_string() }),
            *$crate::rule::STR_ID,
            "Str".to_string(),
        )
    };
    ($name:expr => $s:expr) => {
        $crate::custom!($name => $crate::str!($s))
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rule_handler::Handler;

    #[test]
    fn str_rule_matches_string() {
        let rule = str!("hello");
        let input = "hello";
        let expected_node = Node::new("hello", rule.id, &rule.name);

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn str_rule_does_not_match_different_string() {
        let rule = str!("hello");
        let input = "world";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, None);
    }

    #[test]
    fn str_rule_does_not_match_empty_input() {
        let rule = str!("hello");
        let input = "";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, None);
    }

    #[test]
    fn str_rule_does_not_match_shorter_input() {
        let rule = str!("hello");
        let input = "hell";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, None);
    }

    #[test]
    fn str_rule_does_match_longer_input() {
        let rule = str!("hello");
        let input = "hello world";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        let expected_node = Node::new("hello", rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }
}
