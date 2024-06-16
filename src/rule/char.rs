use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static CHAR_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Char {
    pub c: char,
    pub id: usize,
    pub name: String,
}

impl<'a> Parsable<'a> for Char {
    fn parse(&self, input: &'a str) -> Option<Node<'a>> {
        if input.chars().next() == Some(self.c) {
            Some(Node::new(&input[0..1], self.id, &self.name))
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
macro_rules! char {
    ($c:expr) => {
        crate::rule::Char {
            c: $c,
            id: *crate::rule::CHAR_ID,
            name: "Char".to_string(),
        }
    };
    ($name:expr => $c:expr) => {
        crate::rule::Char {
            c: $c,
            id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            name: $name.to_string(),
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn char_macro_works() {
        let rule = char!('a');
        assert_eq!(rule.c, 'a');
        assert_eq!(rule.name, "Char");
    }

    #[test]
    fn char_macro_unique_ids() {
        let rule1 = char!('a');
        let rule2 = char!('b');
        assert_eq!(rule1.id, rule2.id);
        let rule3 = char!("test" => 'a');
        assert_ne!(rule1.id, rule3.id);
    }

    #[test]
    fn char_rule_matches_single_character() {
        let rule = char!('a');
        let input = "a";
        let expected_node = Node::new("a", rule.id, &rule.name);

        let result = rule.parse(input);

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
