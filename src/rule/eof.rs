use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static EOF_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Eof {
    pub id: usize,
    pub name: String,
}

impl<'a> Parsable<'a> for Eof {
    fn parse(&self, input: &'a str) -> Option<Node<'a>> {
        if input.is_empty() {
            Some(Node::new_empty(self.id, &self.name))
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
macro_rules! eof {
    ($name:expr) => {
        crate::rule::Eof {
            id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            name: $name.to_string(),
        }
    };
    () => {
        crate::rule::Eof {
            id: *crate::rule::EOF_ID,
            name: "Eof".to_string(),
        }
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
