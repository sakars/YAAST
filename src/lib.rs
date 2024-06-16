use std::sync::atomic::AtomicUsize;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)] // Add the Debug trait
pub struct Node<'a> {
    content: &'a str,
    children: Vec<Node<'a>>,
    type_id: usize,
    type_name: String,
}

pub trait Parsable<'a> {
    fn parse(&self, input: &'a str) -> Option<Node<'a>>;
    fn get_id(&self) -> &usize;
    fn get_name(&self) -> &String;
}

impl<'a> Node<'a> {
    pub fn new(content: &'a str, id: usize, name: &String) -> Node<'a> {
        Node {
            content,
            children: Vec::new(),
            type_id: id,
            type_name: name.clone(),
        }
    }

    pub fn new_empty(id: usize, name: &String) -> Node<'a> {
        Node {
            content: "",
            children: Vec::new(),
            type_id: id,
            type_name: name.clone(),
        }
    }

    pub fn add_child(&mut self, child: Node<'a>) {
        self.children.push(child);
    }

    pub fn children(&self) -> &Vec<Node<'a>> {
        &self.children
    }
}

impl std::cmp::PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.children.len() != other.children.len() {
            return false;
        }
        for (a, b) in self.children.iter().zip(other.children.iter()) {
            if a != b {
                return false;
            }
        }
        self.content == other.content && self.children == other.children
    }
}

mod rule {

    use crate::COUNTER;

    use super::Node;
    use super::Parsable;
    use once_cell::sync::Lazy;

    static SOR_ID: Lazy<usize> =
        Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
    static SEQ_ID: Lazy<usize> =
        Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
    static CHAR_ID: Lazy<usize> =
        Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
    static STR_ID: Lazy<usize> =
        Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
    static STAR_ID: Lazy<usize> =
        Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
    static PLUS_ID: Lazy<usize> =
        Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
    static OPT_ID: Lazy<usize> =
        Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
    static EOF_ID: Lazy<usize> =
        Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

    pub struct Sor<'a> {
        options: Vec<Box<dyn Parsable<'a>>>,
        id: usize,
        name: String,
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
            Sor {
                options: vec![$(Box::new($rule)),*],
                id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                name: $name.to_string(),
            }
        };
        ($($rule:expr),*) => {
            Sor {
                options: vec![$(Box::new($rule)),*],
                id: *SOR_ID,
                name: "Sor".to_string(),
            }
        };
    }

    pub struct Seq<'a> {
        rules: Vec<Box<dyn Parsable<'a>>>,
        id: usize,
        name: String,
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
            Seq {
                rules: vec![$(Box::new($rule)),*],
                id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                name: $name.to_string(),
            }
        };
        ($($rule:expr),*) => {
            Seq {
                rules: vec![$(Box::new($rule)),*],
                id: *SEQ_ID,
                name: "Seq".to_string(),
            }
        };
    }

    pub struct Char {
        c: char,
        id: usize,
        name: String,
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
            Char {
                c: $c,
                id: *CHAR_ID,
                name: "Char".to_string(),
            }
        };
        ($name:expr => $c:expr) => {
            Char {
                c: $c,
                id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                name: $name.to_string(),
            }
        };
    }

    pub struct Str {
        s: String,
        id: usize,
        name: String,
    }

    impl<'a> Parsable<'a> for Str {
        fn parse(&self, input: &'a str) -> Option<Node<'a>> {
            if input.starts_with(&self.s) {
                Some(Node::new(&input[0..self.s.len()], self.id, &self.name))
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
    macro_rules! str {
        ($s:expr) => {
            Str {
                s: $s.to_string(),
                id: *STR_ID,
                name: "Str".to_string(),
            }
        };
        ($name:expr => $s:expr) => {
            Str {
                s: $s.to_string(),
                id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                name: $name.to_string(),
            }
        };
    }

    pub struct Star<'a> {
        rule: Box<dyn Parsable<'a>>,
        id: usize,
        name: String,
    }

    impl<'a> Parsable<'a> for Star<'a> {
        fn parse(&self, input: &'a str) -> Option<Node<'a>> {
            let mut node = Node::new_empty(self.id, &self.name);
            let mut size = 0;
            while let Some(child) = self.rule.parse(&input[size..]) {
                size += child.content.len();
                node.add_child(child);
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
    macro_rules! star {
        ($name:expr => $rule:expr) => {
            Star {
                rule: Box::new($rule),
                id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                name: $name.to_string(),
            }
        };
        ($rule:expr) => {
            Star {
                rule: Box::new($rule),
                id: *STAR_ID,
                name: "Star".to_string(),
            }
        };
    }

    pub struct Plus<'a> {
        rule: Box<dyn Parsable<'a>>,
        id: usize,
        name: String,
    }

    impl<'a> Parsable<'a> for Plus<'a> {
        fn parse(&self, input: &'a str) -> Option<Node<'a>> {
            let mut node = Node::new_empty(self.id, &self.name);
            let mut size = 0;
            if let Some(child) = self.rule.parse(input) {
                size += child.content.len();
                node.add_child(child);
            } else {
                return None;
            }
            while let Some(child) = self.rule.parse(&input[size..]) {
                size += child.content.len();
                node.add_child(child);
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
    macro_rules! plus {
        ($name:expr => $rule:expr) => {
            Plus {
                rule: Box::new($rule),
                id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                name: $name.to_string(),
            }
        };
        ($rule:expr) => {
            Plus {
                rule: Box::new($rule),
                id: *PLUS_ID,
                name: "Plus".to_string(),
            }
        };
    }

    pub struct Opt<'a> {
        rule: Box<dyn Parsable<'a>>,
        id: usize,
        name: String,
    }

    impl<'a> Parsable<'a> for Opt<'a> {
        fn parse(&self, input: &'a str) -> Option<Node<'a>> {
            if let Some(node) = self.rule.parse(input) {
                Some(node)
            } else {
                Some(Node::new_empty(self.id, &self.name))
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
    macro_rules! opt {
        ($name:expr => $rule:expr) => {
            Opt {
                rule: Box::new($rule),
                id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                name: $name.to_string(),
            }
        };
        ($rule:expr) => {
            Opt {
                rule: Box::new($rule),
                id: *OPT_ID,
                name: "Opt".to_string(),
            }
        };
    }

    pub struct Eof {
        id: usize,
        name: String,
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
            Eof {
                id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                name: $name.to_string(),
            }
        };
        () => {
            Eof {
                id: *EOF_ID,
                name: "Eof".to_string(),
            }
        };
    }

    #[cfg(test)]
    mod tests {
        mod char {
            use super::super::*;

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

        mod str {
            use super::super::*;

            #[test]
            fn str_rule_matches_string() {
                let rule = str!("hello");
                let input = "hello";
                let expected_node = Node::new("hello", rule.id, &rule.name);

                let result = rule.parse(input);

                assert_eq!(result, Some(expected_node));
            }

            #[test]
            fn str_rule_does_not_match_different_string() {
                let rule = str!("hello");
                let input = "world";

                let result = rule.parse(input);

                assert_eq!(result, None);
            }

            #[test]
            fn str_rule_does_not_match_empty_input() {
                let rule = str!("hello");
                let input = "";

                let result = rule.parse(input);

                assert_eq!(result, None);
            }

            #[test]
            fn str_rule_does_not_match_shorter_input() {
                let rule = str!("hello");
                let input = "hell";

                let result = rule.parse(input);

                assert_eq!(result, None);
            }

            #[test]
            fn str_rule_does_match_longer_input() {
                let rule = str!("hello");
                let input = "hello world";

                let result = rule.parse(input);
                let expected_node = Node::new("hello", rule.id, &rule.name);
                assert_eq!(result, Some(expected_node));
            }
        }

        mod seq {
            use super::super::*;

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

        mod sor {

            use super::super::*;

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

        mod star {
            use super::super::*;

            #[test]
            fn star_rule_matches_zero_times() {
                let rule = star!(char!('a'));
                let input = "";

                let result = rule.parse(input);

                let expected_node = Node::new_empty(rule.id, &rule.name);
                assert_eq!(result, Some(expected_node));
            }

            #[test]
            fn star_rule_matches_multiple_times() {
                let rule = star!(char!('a'));
                let input = "aaa";
                let mut expected_node = Node::new("aaa", rule.id, &rule.name);
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));

                let result = rule.parse(input);

                assert_eq!(result, Some(expected_node));
            }

            #[test]
            fn star_rule_matches_longer_input() {
                let rule = star!(char!('a'));
                let input = "aaaab";
                let mut expected_node = Node::new("aaaa", *rule.get_id(), rule.get_name());
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));

                let result = rule.parse(input);
                assert_eq!(result, Some(expected_node));
            }
        }

        mod plus {
            use super::super::*;

            #[test]
            fn plus_rule_matches_one_time() {
                let rule = plus!(char!('a'));
                let input = "a";

                let result = rule.parse(input);

                let mut expected_node = Node::new("a", *rule.get_id(), rule.get_name());
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
                assert_eq!(result, Some(expected_node));
            }

            #[test]
            fn plus_rule_matches_multiple_times() {
                let rule = plus!(char!('a'));
                let input = "aaa";
                let mut expected_node = Node::new("aaa", *rule.get_id(), rule.get_name());
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));

                let result = rule.parse(input);

                assert_eq!(result, Some(expected_node));
            }

            #[test]
            fn plus_rule_does_not_match_zero_times() {
                let rule = plus!(char!('a'));
                let input = "";

                let result = rule.parse(input);

                assert_eq!(result, None);
            }

            #[test]
            fn plus_rule_matches_longer_input() {
                let rule = plus!(char!('a'));
                let input = "aaaab";
                let mut expected_node = Node::new("aaaa", *rule.get_id(), rule.get_name());
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));
                expected_node.add_child(Node::new("a", *rule.get_id(), rule.get_name()));

                let result = rule.parse(input);
                assert_eq!(result, Some(expected_node));
            }
        }

        mod eof {
            use super::super::*;

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
    }
}
