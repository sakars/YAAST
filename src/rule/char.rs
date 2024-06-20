use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static CHAR_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct One {
    pub c: char,
}

impl<'a> Parsable<'a> for One {
    fn parse(&self, input: &'a str, id: usize, name: &String) -> Option<Node<'a>> {
        if input.chars().next() == Some(self.c) {
            Some(Node::new(&input[0..1], id, &name))
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! char {
    ($c:expr) => {
        $crate::rule::Rule::new(
            Box::new($crate::rule::One { c: $c }),
            *$crate::rule::CHAR_ID,
            "Char".to_string()
        )
    };
    ($name:expr => $c:expr) => {
        $crate::custom!($name => $crate::char!($c))
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn char_macro_works() {
        let rule = char!('a');
        let tree = rule.parse("a");
        assert_eq!(tree.as_ref().unwrap().content, "a");
        assert_eq!(tree.as_ref().unwrap().type_name, "Char");
        let rule = char!("test" => 'a');
        let tree = rule.parse("a");
        assert_eq!(tree.as_ref().unwrap().content, "a");
        assert_eq!(tree.as_ref().unwrap().type_name, "test");
        assert_eq!(tree.as_ref().unwrap().children.len(), 1);
        assert_eq!(tree.as_ref().unwrap().children[0].content, "a");
    }

    #[test]
    fn parsing_is_deterministic() {
        let rule1 = char!('a');
        let node1 = rule1.parse("a");
        let node2 = rule1.parse("a");
        assert_eq!(node1, node2);
    }

    #[test]
    fn char_macro_unique_ids() {
        let rule1 = char!('a');
        let rule2 = char!('a');
        let node1 = rule1.parse("a");
        let node2 = rule2.parse("a");
        assert_eq!(node1.as_ref().unwrap().type_id, node2.unwrap().type_id);
        assert_eq!(rule1.id, rule2.id);
        assert_eq!(rule1.id, *super::CHAR_ID);
        assert_eq!(rule2.id, *super::CHAR_ID);
        assert_eq!(rule1.id, node1.as_ref().unwrap().type_id);
    }

    #[test]
    fn char_rule_matches_single_character() {
        let rule = char!('a');
        let input = "a";
        let result = rule.parse(input);
        let expected_node = Node::new("a", rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn char_rule_does_not_match_different_character() {
        let rule = char!('a');
        let input = "b";

        let result = rule.parse(input);

        assert_eq!(result, None);
    }

    #[test]
    fn char_rule_does_not_match_empty_input() {
        let rule = char!('a');
        let input = "";

        let result = rule.parse(input);

        assert_eq!(result, None);
    }

    #[test]
    fn char_rule_does_match_longer_input() {
        let rule = char!('a');
        let input = "abc";

        let result = rule.parse(input);
        let expected_node = Node::new("a", rule.id, &rule.name);
        assert_eq!(result, Some(expected_node));
    }
}
