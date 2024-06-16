pub mod node;
pub use node::*;
#[macro_use]
pub mod rule;

#[cfg(test)]
mod tests {
    use crate::Parsable;

    #[test]
    fn parse_expression() {
        let identifier = seq!("identifier" =>
        sor!(
            char!('_'),
            ranges!(('a','z'), ('A','Z'))
        ),
        star!(
            sor!(
                char!('_'),
                ranges!(('a','z'), ('A','Z'), ('0','9'))
            )
        )
        );
        let res = identifier.parse("a_1");
        assert_eq!(res.is_some(), true);
        let node = res.unwrap();
        assert_eq!(node.content, "a_1");
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].content, "a");
        assert_eq!(node.children[1].content, "_1");
        assert_eq!(node.children[1].children.len(), 2);
        assert_eq!(node.children[1].children[0].content, "_");
        assert_eq!(node.children[1].children[1].content, "1");
    }
}
