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
    use rule::CHAR_ID;

    use crate::*;

    #[test]
    fn opt_rule_matches_zero_times() {
        let rule = opt!(char!('a'));
        let input = "";

        let result = rule.parse(input);

        let expected_node = Node::new_empty(rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn opt_rule_matches_one_time() {
        let rule = opt!(char!('a'));
        let input = "a";

        let result = rule.parse(input);

        let expected_node = Node::new("a", rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn opt_rule_matches_one_time_with_custom_name() {
        let rule = opt!("test" => char!('a'));
        let input = "a";

        let result = rule.parse(input);

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
        let expected_node = Node::new_empty(rule.id, &rule.name);

        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn opt_rule_matches_only_first_character() {
        let rule = opt!(char!('a'));
        let input = "aa";

        let result = rule.parse(input);

        let expected_node = Node::new("a", rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }
}
