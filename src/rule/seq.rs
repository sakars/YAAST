use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static SEQ_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Seq<'a> {
    pub rules: Vec<Box<dyn Parsable<'a>>>,
    pub id: usize,
    pub name: String,
}

impl<'a> Parsable<'a> for Seq<'a> {
    fn parse(&self, input: &'a str) -> Option<Node<'a>> {
        let mut node = Node::new_empty(self.id, &self.name);
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
    fn get_id(&self) -> &usize {
        &self.id
    }
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[macro_export]
macro_rules! seq {
        ($name:expr => $($rule:expr),*) => {
            crate::rule::Seq {
                rules: vec![$(Box::new($rule)),*],
                id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                name: $name.to_string(),
            }
        };
        ($($rule:expr),*) => {
            crate::rule::Seq {
                rules: vec![$(Box::new($rule)),*],
                id: *crate::rule::SEQ_ID,
                name: "Seq".to_string(),
            }
        };
    }

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn seq_rule_matches_multiple_rules() {
        let rule = seq!(char!('a'), char!('b'));
        let input = "ab";
        let mut expected_node = Node::new("ab", rule.id, &rule.name);
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
        expected_node.add_child(Node::new("b", *rule.get_id(), rule.get_name()));

        let result = rule.parse(input);

        assert_eq!(result, Some(expected_node));
    }

    #[test]
    fn seq_rule_does_not_match_partial_input() {
        let rule = seq!(char!('a'), char!('b'));
        let input = "a";

        let result = rule.parse(input);

        assert_eq!(result, None);
    }

    #[test]
    fn seq_rule_does_not_match_empty_input() {
        let rule = seq!(char!('a'), char!('b'));
        let input = "";

        let result = rule.parse(input);

        assert_eq!(result, None);
    }

    #[test]
    fn seq_rule_does_match_longer_input() {
        let rule = seq!(char!('a'), char!('b'));
        let input = "abc";

        let result = rule.parse(input);
        let mut expected_node = Node::new("ab", rule.id, &rule.name);
        expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
        expected_node.add_child(Node::new("b", *rule.get_id(), rule.get_name()));
        assert_eq!(result, Some(expected_node));
    }
}
