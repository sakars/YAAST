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
    use crate::*;

    #[test]
    fn eof_rule_matches_empty_input() {
        let rule = eof!();
        let input = "";

        let result = rule.parse(input);

        let expected_node = Node::new_empty(rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn eof_rule_does_not_match_non_empty_input() {
        let rule = eof!();
        let input = "a";

        let result = rule.parse(input);

        assert_eq!(result, None);
    }

    #[test]
    fn eof_rule_does_not_match_longer_input() {
        let rule = eof!();
        let input = "a";

        let result = rule.parse(input);

        assert_eq!(result, None);
    }
}
