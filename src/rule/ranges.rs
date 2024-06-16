use crate::COUNTER;

use crate::Node;
use crate::Parsable;
use once_cell::sync::Lazy;

pub static RANGES_ID: Lazy<usize> =
    Lazy::new(|| COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

pub struct Ranges {
    pub ranges: Vec<(char, char)>,
    pub id: usize,
    pub name: String,
}

impl<'a> Parsable<'a> for Ranges {
    fn parse(&self, input: &'a str) -> Option<Node<'a>> {
        if let Some(c) = input.chars().next() {
            for (start, end) in &self.ranges {
                if c >= *start && c <= *end {
                    return Some(Node::new(&input[0..1], self.id, &self.name));
                }
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
macro_rules! ranges {
    ($name:expr => $ranges:expr) => {
        crate::rule::Ranges {
            ranges: vec![$ranges],
            id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            name: $name.to_string(),
        }
    };
    ($($ranges:expr), *) => {
        crate::rule::Ranges {
            ranges: vec![$($ranges), *],
            id: *crate::rule::RANGES_ID,
            name: "Ranges".to_string(),
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ranges_macro_works() {
        let rule = ranges!(('a', 'z'));
        assert_eq!(rule.ranges, vec![('a', 'z')]);
        assert_eq!(rule.name, "Ranges");
    }

    #[test]
    fn ranges_matching_alphanumerics() {
        let rule = star!(ranges!(('a', 'z'), ('A', 'Z'), ('0', '9')));
        let input = "agfdaohehslh83945nklg2DFKASGH252GIRO";
        let expected_node = Node::new(input, rule.id, &rule.name);
        let result = rule.parse(input);
        assert!(result.is_some());
        let node = result.as_ref().unwrap();
        assert_eq!(node.content, expected_node.content);
        assert_eq!(node.children.len(), input.len());
        for (i, c) in input.chars().enumerate() {
            assert_eq!(node.children[i].content, c.to_string());
        }
    }
}
