use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static EOF_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Eof {}

impl<'a> Parsable<'a> for Eof {
    fn parse(&self, input: &'a str, id: usize, name: &String) -> Option<Node<'a>> {
        if input.is_empty() {
            Some(Node::new_empty(id, &name))
        } else {
            None
        }
    }
    fn parse_with_handler(
        &self,
        input: &'a str,
        id: usize,
        name: &String,
        handler: &crate::rule_handler::Handler<'a>,
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
macro_rules! eof {
    () => {
        $crate::rule::Rule::new(
            Box::new($crate::rule::Eof { }),
            *$crate::rule::EOF_ID,
            "Eof".to_string()
        )
    };
    ($name:expr) => {
        $crate::custom!($name => $crate::eof!())
    };
}

#[cfg(test)]
mod tests {
    use rule_handler::Handler;

    use crate::*;

    #[test]
    fn eof_rule_matches_empty_input() {
        let rule = eof!();
        let input = "";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        let expected_node = Node::new_empty(rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn eof_rule_does_not_match_non_empty_input() {
        let rule = eof!();
        let input = "a";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, None);
    }

    #[test]
    fn eof_rule_does_not_match_longer_input() {
        let rule = eof!();
        let input = "ahdgfhfgh";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, None);
    }
}
