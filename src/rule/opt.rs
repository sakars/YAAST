use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

use super::Rule;

pub static OPT_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Opt<'a> {
    pub rule: Rule<'a>,
}

impl<'a> Parsable<'a> for Opt<'a> {
    fn parse(&self, input: &'a str, id: usize, name: &String) -> Option<Node<'a>> {
        if let Some(node) = self.rule.parse(input) {
            Some(node)
        } else {
            Some(Node::new_empty(id, &name))
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
        if let Some(mut success) = self.rule.parse_with_handler(input, handler) {
            handler.handle_success(&mut success);
            Some(success)
        } else {
            let mut failure = Node::new_empty(id, name);
            handler.handle_success(&mut failure);
            Some(failure)
        }
    }
}

#[macro_export]
macro_rules! opt {
    ($name:expr => $rule:expr) => {
        $crate::custom!($name => $crate::opt!($rule))
    };
    ($rule:expr) => {
        $crate::rule::Rule::new(
            Box::new($crate::rule::Opt { rule: $rule }),
            *$crate::rule::OPT_ID,
            "Opt".to_string()
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rule_handler::Handler;

    #[test]
    fn opt_rule_matches_zero_times() {
        let rule = opt!(char!('a'));
        let input = "";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        let expected_node = Node::new_empty(rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn opt_rule_matches_one_time() {
        let rule = opt!(char!('a'));
        let input = "a";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        let expected_node = Node::new("a", rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn opt_rule_matches_one_time_with_custom_name() {
        let rule = opt!("test" => char!('a'));
        let input = "a";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        let mut expected_node = Node::new("a", rule.id, &"test".to_string());
        expected_node
            .children
            .push(Node::new("a", *crate::rule::CHAR_ID, &"Char".to_string()));
        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn opt_rule_matches_incompatible_input() {
        let rule = opt!(char!('a'));
        let input = "b";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        let expected_node = Node::new_empty(rule.id, &rule.name);

        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn opt_rule_matches_only_first_character() {
        let rule = opt!(char!('a'));
        let input = "aa";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        let expected_node = Node::new("a", rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }
}
