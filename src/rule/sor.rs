use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static SOR_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Sor<'a> {
    pub options: Vec<crate::rule::Rule<'a>>,
}

impl<'a> Parsable<'a> for Sor<'a> {
    fn parse(&self, input: &'a str, id: usize, name: &String) -> Option<Node<'a>> {
        for rule in &self.options {
            if let Some(node) = rule.parse(input) {
                let mut sor_node = Node::new(node.content, id, &name);
                sor_node.add_child(node);
                return Some(sor_node);
            }
        }
        None
    }

    fn parse_with_handler(
        &self,
        input: &'a str,
        id: usize,
        name: &String,
        handler: &crate::rule_handler::Handler<'a>,
    ) -> Option<Node<'a>> {
        handler.handle_pre_parse(id);
        for rule in &self.options {
            if let Some(node) = rule.parse_with_handler(input, handler) {
                let mut sor_node = Node::new(node.content, id, name);
                sor_node.add_child(node);
                handler.handle_success(&mut sor_node);
                return Some(sor_node);
            }
        }
        handler.handle_failure(id);
        None
    }
}

#[macro_export]
macro_rules! sor {
    ($name:expr => $($rule:expr),*) => {
        $crate::custom!($name => $crate::sor!($($rule),*))
    };
    ($($rule:expr),*) => {
        crate::rule::Rule::new(
            Box::new(crate::rule::Sor {
                options: vec![$($rule.clone()),*],
            }),
            *crate::rule::SOR_ID,
            "Sor".to_string(),
        )
    };

}

#[cfg(test)]
mod tests {
    use crate::*;
    use rule::CHAR_ID;
    use rule_handler::Handler;

    #[test]
    fn macro_sor_works() {
        let rule = sor!("test" =>
            sor!(),
            sor!()
        );
        assert_eq!(rule.name, "test");
    }

    #[test]
    fn sor_rule_matches_first_rule() {
        let rule = sor!(char!('a'), char!('b'));
        let input = "a";
        let mut sor_node = Node::new("a", rule.id, &rule.name);
        sor_node.add_child(Node::new("a", *CHAR_ID, &"Char".to_string()));

        let result = rule.parse(input);

        assert_eq!(result, Some(sor_node));
    }

    #[test]
    fn sor_rule_matches_second_rule() {
        let rule = sor!(char!('a'), char!('b'));
        let input = "b";
        let mut sor_node = Node::new("b", rule.id, &rule.name);
        sor_node.add_child(Node::new("b", *CHAR_ID, &"Char".to_string()));

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, Some(sor_node));
    }

    #[test]
    fn sor_rule_does_not_match_empty_input() {
        let rule = sor!(char!('a'), char!('b'));
        let input = "";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, None);
    }

    #[test]
    fn sor_rule_does_not_match_no_rule() {
        let rule = sor!();
        let input = "a";

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, None);
    }

    #[test]
    fn sor_rule_does_match_longer_input() {
        let rule = sor!(char!('a'), char!('b'));
        let input = "abc";
        let mut sor_node = Node::new("a", rule.id, &rule.name);
        sor_node.add_child(Node::new("a", *CHAR_ID, &"Char".to_string()));

        let result = rule.parse(input);
        let result2 = rule.parse_with_handler(input, &Handler::new());
        assert_eq!(result, result2);

        assert_eq!(result, Some(sor_node));
    }
}
