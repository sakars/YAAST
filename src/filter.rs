use crate::Node;

pub struct Filter {
    pub allow_by_default: bool,
    pub exceptions: Vec<usize>,
}

impl Filter {
    pub fn new(allow_by_default: bool) -> Filter {
        Filter {
            allow_by_default,
            exceptions: Vec::new(),
        }
    }

    pub fn new_with_list(allow_by_default: bool, exceptions: Vec<usize>) -> Filter {
        Filter {
            allow_by_default,
            exceptions,
        }
    }

    pub fn add_filter(&mut self, filter: usize) {
        self.exceptions.push(filter);
    }

    pub fn remove_filter(&mut self, filter: usize) {
        self.exceptions.retain(|&x| x != filter);
    }

    pub fn is_allowed(&self, filter: usize) -> bool {
        if !self.allow_by_default {
            self.exceptions.contains(&filter)
        } else {
            !self.exceptions.contains(&filter)
        }
    }
}

impl<'a> Filter {
    /// Filters the AST based on the allowlist.
    /// It will remove all nodes that are not in the allowlist.
    /// If the ast has no root node, it will create one.
    /// That is necessary because the filter needs a root node to work,
    /// otherwise filtering could result in a tree with multiple roots.
    pub fn filter_ast(&self, ast: Node<'a>) -> Node<'a> {
        let is_root = ast.type_id == *crate::ROOT_ID;
        let mut root = if is_root { ast } else { Node::new_as_root(ast) };
        self.filter_node(&mut root);
        return root;
    }

    fn filter_node(&self, node: &mut Node<'a>) -> () {
        while self.filter_node_children(node) {}
        for child in &mut node.children {
            self.filter_node(child);
        }
    }

    fn filter_node_children(&self, node: &mut Node<'a>) -> bool {
        let mut new_children = Vec::new();
        let mut did_extract = false;
        for child in &mut node.children {
            if !self.is_allowed(child.type_id) {
                new_children = self.extract_children(child, new_children);
                did_extract = true;
            } else {
                new_children.push(std::mem::replace(child, Node::new_as_unreachable()));
            }
        }
        node.children = new_children;
        did_extract
    }

    fn extract_children(&self, node: &mut Node<'a>, mut vec: Vec<Node<'a>>) -> Vec<Node<'a>> {
        let mut children = std::mem::replace(&mut node.children, Vec::new());
        vec.append(&mut children);
        vec
    }
}

#[cfg(test)]
mod tests {

    use rule::{CHAR_ID, SEQ_ID, SOR_ID, STAR_ID};

    use crate::*;

    #[test]
    fn allowlist_filter_ast_only_chars() {
        let r = star!(sor!(seq!(char!('a'), char!('b')), char!('c')));
        let input = "abcab";
        let ast = r.parse(input);
        let filter = Filter::new_with_list(false, vec![*CHAR_ID]);
        let filtered_ast = filter.filter_ast(ast.unwrap());
        assert_eq!(filtered_ast.type_id, *ROOT_ID);
        assert_eq!(filtered_ast.children.len(), 5);
        for child in &filtered_ast.children {
            assert_eq!(child.type_id, *CHAR_ID);
        }
        for (char, child) in input.chars().zip(filtered_ast.children.iter()) {
            assert_eq!(char.to_string(), child.content);
        }
    }

    #[test]
    fn allowlist_filter_ast_non_leaf_nodes() {
        let r = star!(sor!(seq!(char!('a'), char!('b')), char!('c')));
        let input = "abcab";
        let ast = r.parse(input);
        let filter = Filter::new_with_list(false, vec![*SEQ_ID]);
        let filtered_ast = filter.filter_ast(ast.unwrap());
        assert_eq!(filtered_ast.type_id, *ROOT_ID);
        assert_eq!(filtered_ast.children.len(), 2);
        for child in &filtered_ast.children {
            assert_eq!(child.type_id, *SEQ_ID);
            assert_eq!(child.children.len(), 0);
            assert_eq!(child.content, "ab");
        }
    }

    #[test]
    fn allowlist_empty() {
        let r = star!(sor!(seq!(char!('a'), char!('b')), char!('c')));
        let input = "abcab";
        let ast = r.parse(input);
        let filter = Filter::new_with_list(false, vec![]);
        let filtered_ast = filter.filter_ast(ast.unwrap());
        assert_eq!(filtered_ast.type_id, *ROOT_ID);
        // Empty allowlist means only the root node remains
        assert_eq!(filtered_ast.children.len(), 0);
    }

    #[test]
    fn allowlist_custom_rules() {
        let b = char!("B" => 'b');
        let r = star!("myRule" => sor!(seq!(char!('a'), b), char!('c')));
        let input = "abcab";
        let ast = r.parse(input);
        //println!("{:?}", ast);
        let filter = Filter::new_with_list(false, vec![b.id, r.id]);
        let filtered_ast = filter.filter_ast(ast.unwrap());
        // println!("root id: {}", *ROOT_ID);
        // println!("r.id: {}", r.id);
        // println!("b.id: {}", b.id);
        // println!("{:?}", filtered_ast);
        assert_eq!(filtered_ast.type_id, *ROOT_ID);
        let child = &filtered_ast.children[0];
        assert_eq!(child.type_id, r.id);
        assert_eq!(child.type_name, "myRule");
        assert_eq!(child.content, "abcab");
        assert_eq!(child.children.len(), 2);
        assert_eq!(child.children[0].type_id, b.id);
        assert_eq!(child.children[1].type_id, b.id);
        assert_eq!(child.children[0].content, "b");
        assert_eq!(child.children[1].content, "b");
    }

    #[test]
    fn blocklist_filter_ast_only_chars() {
        let r = star!(sor!(seq!(char!('a'), char!('b')), char!('c')));
        let input = "abcab";
        let ast = r.parse(input);
        let filter = Filter::new_with_list(true, vec![*CHAR_ID]);
        let filtered_ast = filter.filter_ast(ast.unwrap());
        // println!("{:?}", filtered_ast);
        assert_eq!(filtered_ast.type_id, *ROOT_ID);
        assert_eq!(filtered_ast.children.len(), 1);
        let child = &filtered_ast.children[0];
        assert_eq!(child.type_id, *STAR_ID);
        assert_eq!(child.children.len(), 3);
        let c1 = &child.children[0];
        let c2 = &child.children[1];
        let c3 = &child.children[2];
        assert_eq!(c1.type_id, *SOR_ID);
        assert_eq!(c2.type_id, *SOR_ID);
        assert_eq!(c3.type_id, *SOR_ID);
        assert_eq!(c1.children.len(), 1);
        assert_eq!(c2.children.len(), 0);
        assert_eq!(c3.children.len(), 1);
        assert_eq!(c1.content, "ab");
        assert_eq!(c2.content, "c");
        assert_eq!(c3.content, "ab");
        let c1 = &c1.children[0];
        let c3 = &c3.children[0];
        assert_eq!(c1.type_id, *SEQ_ID);
        assert_eq!(c3.type_id, *SEQ_ID);
        assert_eq!(c1.children.len(), 0);
        assert_eq!(c3.children.len(), 0);
        assert_eq!(c1.content, "ab");
        assert_eq!(c3.content, "ab");
    }
}
