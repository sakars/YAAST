pub mod node;
pub use node::*;
#[macro_use]
pub mod rule;

pub mod filter;
pub use filter::*;

pub mod rule_handler;

#[cfg(test)]
mod tests {

    #[test]
    fn back_and_forth() {
        let a_name = String::from("ARule");
        let b_name = String::from("BRule");
        let a = custom!(a_name);
        let b = custom!(b_name);
        let a = a.init(sor!(seq!(char!('a'), b.get()), char!('b')));
        let b = b.init(sor!(seq!(char!('b'), a.get()), char!('a')));
        let res = a.parse("abb");
        let res2 = b.parse("baa");
        let res3 = a.parse("ababababaa");
        let res4 = a.parse("ababababa");
        assert!(res.is_some());
        assert!(res2.is_some());
        assert!(res3.is_some());
        assert!(res4.is_none());
        let n = res3.as_ref().unwrap();
        assert_eq!(n.content, "ababababaa");
        assert_eq!(n.type_name, a_name);
        assert_eq!(n.children().len(), 1);
        let n = &n.children()[0];
        assert_eq!(n.content, "ababababaa");
        assert_eq!(n.type_name, "Sor");
        assert_eq!(n.children().len(), 1);
        let n = &n.children()[0];
        assert_eq!(n.children().len(), 2);
        assert_eq!(n.type_name, "Seq");
        let n1 = &n.children()[0];
        let n2 = &n.children()[1];
        assert_eq!(n1.content, "a");
        assert_eq!(n1.type_name, "Char");
        assert_eq!(n1.children().len(), 0);
        assert_eq!(n2.content, "babababaa");
        assert_eq!(n2.type_name, b_name);
        assert_eq!(n2.children().len(), 1);
        let n = &n2.children()[0];
        assert_eq!(n.content, "babababaa");
        assert_eq!(n.type_name, "Sor");
        assert_eq!(n.children().len(), 1);
        let n = &n.children()[0];
        assert_eq!(n.children().len(), 2);
        assert_eq!(n.type_name, "Seq");
        assert_eq!(n.content, "babababaa");
        let n1 = &n.children()[0];
        let n2 = &n.children()[1];
        assert_eq!(n1.content, "b");
        assert_eq!(n2.content, "abababaa");
        assert_eq!(n1.type_name, "Char");
        assert_eq!(n2.type_name, a_name);
        assert_eq!(n1.children().len(), 0);
        assert_eq!(n2.children().len(), 1);
    }
}
