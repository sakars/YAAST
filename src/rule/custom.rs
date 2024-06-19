use crate::COUNTER;

use super::Rule;
use crate::Node;
use crate::Parsable;

pub struct Custom<'a> {
    pub rule: Rule<'a>,
}

impl<'a> Parsable<'a> for Custom<'a> {
    fn parse(&self, input: &'a str, id: usize, name: &String) -> Option<Node<'a>> {
        // Forwards the parse call to the rule stored in the Custom struct
        // and wraps the result in its own Node struct
        // This is useful for filtering out unwanted nodes
        if let Some(node) = self.rule.parse(input) {
            let mut wrapper = Some(Node::new(node.content, id, name));
            wrapper.as_mut().unwrap().add_child(node);
            wrapper
        } else {
            None
        }
    }
}

impl<'a> Custom<'a> {
    pub fn new(rule: Rule<'a>) -> Custom<'a> {
        Custom { rule }
    }

    pub fn new_rule(rule: Rule<'a>, name: String) -> Rule<'a> {
        let custom = Custom::<'a>::new(rule);
        let x = Box::<Custom<'a>>::new(custom);
        Rule::<'a>::new(
            x,
            COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            name,
        )
    }
}

pub struct UninitializedRule<'a> {
    rule: Rule<'a>,
}

impl<'a> UninitializedRule<'a> {
    pub fn new(name: String) -> UninitializedRule<'a> {
        UninitializedRule {
            rule: Rule::new_late_instantiated(name),
        }
    }

    pub fn init(&self, rule: Rule<'a>) -> Rule<'a> {
        if let Err(_) = self.rule.rule.set(Box::new(Custom::new(rule))) {
            panic!("Rule already initialized")
        } else {
            return self.rule.clone();
        }
    }

    pub fn get(&self) -> Rule<'a> {
        self.rule.clone()
    }
}

#[macro_export]
macro_rules! custom {
    ($name:expr => $rule:expr) => {
        $crate::rule::Custom::new_rule($rule, $name.to_string())
    };
    ($name:expr) => {
        $crate::rule::UninitializedRule::new($name.to_string())
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn custom_macro_works() {
        let rule = custom!("Custom" => char!('a'));
        let input = "a";
        let node = rule.parse(input).unwrap();
        assert_eq!(node.content, "a");
        assert_ne!(node.type_id, *rule::CHAR_ID);
        assert_eq!(node.type_name, "Custom");
        assert_eq!(node.children().len(), 1);
        assert!(node.children()[0].content == "a");
        assert!(node.children()[0].type_id == *rule::CHAR_ID);
    }

    #[test]
    fn custom_macro_works_with_seq() {
        let rule = custom!("Custom" => seq!(char!('a'), char!('b')));
        let input = "ab";
        let node = rule.parse(input).unwrap();
        assert_eq!(node.content, "ab");
        assert_ne!(node.type_id, *rule::SEQ_ID);
        assert_eq!(node.type_name, "Custom");
        assert_eq!(node.children().len(), 1);
        assert_eq!(node.children()[0].content, "ab");
        assert_eq!(node.children()[0].type_id, *rule::SEQ_ID);
        assert_eq!(node.children()[0].children().len(), 2);
        assert_eq!(node.children()[0].children()[0].content, "a");
        assert_eq!(node.children()[0].children()[0].type_id, *rule::CHAR_ID);
        assert_eq!(node.children()[0].children()[1].content, "b");
        assert_eq!(node.children()[0].children()[1].type_id, *rule::CHAR_ID);
    }

    #[test]
    #[should_panic(expected = "Rule not initialized")]
    fn custom_macro_uninitialized_rule_panics() {
        let rule = custom!("Custom");
        let input = "a";
        // This should panic because the rule is not initialized
        let _node = rule.rule.parse(input);
    }

    #[test]
    fn latant_init_for_custom_rule() {
        let rule = custom!("Custom");
        let rule = rule.init(char!('a'));
        let input = "a";
        let node = rule.parse(input).unwrap();
        assert_eq!(node.content, "a");
        assert_ne!(node.type_id, *rule::CHAR_ID);
        assert_eq!(node.type_name, "Custom");
        assert_eq!(node.children().len(), 1);
        assert_eq!(node.children()[0].content, "a");
        assert_eq!(node.children()[0].type_id, *rule::CHAR_ID);
        assert_eq!(node.children()[0].type_name, "Char");
    }
}
