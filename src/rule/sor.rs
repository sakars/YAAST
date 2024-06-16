use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static SOR_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Sor<'a> {
    pub options: Vec<Box<dyn Parsable<'a>>>,
    pub id: usize,
    pub name: String,
}

impl<'a> Parsable<'a> for Sor<'a> {
    fn parse(&self, input: &'a str) -> Option<Node<'a>> {
        for rule in &self.options {
            if let Some(node) = rule.parse(input) {
                let mut sor_node = Node::new(node.content, self.id, &self.name);
                sor_node.add_child(node);
                return Some(sor_node);
            }
        }
        None
    }
    fn get_id(&self) -> &usize {
        &self.id
    }
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[macro_export]
macro_rules! sor {
        ($name:expr => $($rule:expr),*) => {
            crate::rule::Sor {
                options: vec![$(Box::new($rule)),*],
                id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                name: $name.to_string(),
            }
        };
        ($($rule:expr),*) => {
            crate::rule::Sor {
                options: vec![$(Box::new($rule)),*],
                id: *crate::rule::SOR_ID,
                name: "Sor".to_string(),
            }
        };
    }

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn macro_sor_works() {
        let rule = sor!("test" =>
            sor!(),
            sor!()
        );
        assert_eq!(rule.options.len(), 2);
        assert_eq!(rule.name, "test");
        let id = rule.get_id();
        let id0 = rule.options[0].get_id();
        let id1 = rule.options[1].get_id();
        assert_eq!(id0, id1);
        assert_ne!(id, id0);
        assert_ne!(id, id1);
        let name = rule.get_name();
        let name0 = rule.options[0].get_name();
        let name1 = rule.options[1].get_name();
        assert_eq!(name, "test");
        assert_eq!(name0, "Sor");
        assert_eq!(name1, "Sor");
    }

    #[test]
    fn sor_rule_matches_first_rule() {
        let rule = sor!(char!('a'), char!('b'));
        let input = "a";
        let mut sor_node = Node::new("a", rule.id, &rule.name);
        sor_node.add_child(Node::new(
            "a",
            *rule.options[0].get_id(),
            rule.options[0].get_name(),
        ));

        let result = rule.parse(input);

        assert_eq!(result, Some(sor_node));
    }

    #[test]
    fn sor_rule_matches_second_rule() {
        let rule = sor!(char!('a'), char!('b'));
        let input = "b";
        let mut sor_node = Node::new("b", rule.id, &rule.name);
        sor_node.add_child(Node::new(
            "b",
            *rule.options[1].get_id(),
            rule.options[1].get_name(),
        ));

        let result = rule.parse(input);

        assert_eq!(result, Some(sor_node));
    }

    #[test]
    fn sor_rule_does_not_match_empty_input() {
        let rule = sor!(char!('a'), char!('b'));
        let input = "";

        let result = rule.parse(input);

        assert_eq!(result, None);
    }

    #[test]
    fn sor_rule_does_not_match_no_rule() {
        let rule = sor!();
        let input = "a";

        let result = rule.parse(input);

        assert_eq!(result, None);
    }

    #[test]
    fn sor_rule_does_match_longer_input() {
        let rule = sor!(char!('a'), char!('b'));
        let input = "abc";
        let mut sor_node = Node::new("a", rule.id, &rule.name);
        sor_node.add_child(Node::new(
            "a",
            *rule.options[0].get_id(),
            rule.options[0].get_name(),
        ));

        let result = rule.parse(input);

        assert_eq!(result, Some(sor_node));
    }
}
