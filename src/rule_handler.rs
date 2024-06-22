use std::collections::HashMap;

use crate::Node;

type PreParseFn<'a> = fn() -> ();
type SuccessFn<'a> = fn(&mut Node<'a>) -> ();
type FailureFn<'a> = fn() -> ();

pub struct Handler<'a> {
    pre_parse_map: HashMap<usize, Vec<PreParseFn<'a>>>,
    success_map: HashMap<usize, Vec<SuccessFn<'a>>>,
    failure_map: HashMap<usize, Vec<FailureFn<'a>>>,
}

impl<'a> Handler<'a> {
    pub fn new() -> Handler<'a> {
        Handler {
            success_map: HashMap::new(),
            failure_map: HashMap::new(),
            pre_parse_map: HashMap::new(),
        }
    }

    pub fn add_success_handler(&mut self, id: usize, handler: SuccessFn<'a>) {
        if let Some(vec) = self.success_map.get_mut(&id) {
            vec.push(handler);
        } else {
            self.success_map.insert(id, vec![handler]);
        }
    }

    pub fn add_failure_handler(&mut self, id: usize, handler: FailureFn<'a>) {
        if let Some(vec) = self.failure_map.get_mut(&id) {
            vec.push(handler);
        } else {
            self.failure_map.insert(id, vec![handler]);
        }
    }

    pub fn add_pre_parse_handler(&mut self, id: usize, handler: PreParseFn<'a>) {
        if let Some(vec) = self.pre_parse_map.get_mut(&id) {
            vec.push(handler);
        } else {
            self.pre_parse_map.insert(id, vec![handler]);
        }
    }

    pub fn handle_success(&self, node: &mut Node<'a>) {
        if let Some(vec) = self.success_map.get(&node.type_id) {
            for handler in vec {
                handler(node);
            }
        }
    }

    pub fn handle_failure(&self, type_id: usize) {
        if let Some(vec) = self.failure_map.get(&type_id) {
            for handler in vec {
                handler();
            }
        }
    }

    pub fn handle_pre_parse(&self, type_id: usize) {
        if let Some(vec) = self.pre_parse_map.get(&type_id) {
            for handler in vec {
                handler();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rule_handler::Handler;

    use crate::*;

    fn success_fn(node: &mut Node) {
        node.content = "Success";
    }

    fn failure_fn() {
        panic!("Failure");
    }

    #[test]
    fn handler_adds_success_handler() {
        let mut handler = Handler::new();
        let id = 1;
        let mut node = Node::new_empty(id, &"Test".to_string());

        handler.add_success_handler(id, success_fn);
        handler.handle_success(&mut node);

        assert_eq!(node.content, "Success");
    }

    #[test]
    #[should_panic(expected = "Failure")]
    fn handler_adds_failure_handler() {
        let mut handler = Handler::new();
        let id = 1;
        handler.add_failure_handler(id, failure_fn);
        handler.handle_failure(id);
    }

    #[test]
    #[should_panic(expected = "Pre-parse")]
    fn handler_adds_pre_parse_handler() {
        let mut handler = Handler::new();
        let id = 1;
        let mut node = Node::new_empty(id, &"Test".to_string());
        let pre_parse_fn = || {
            panic!("Pre-parse");
        };

        handler.add_pre_parse_handler(id, pre_parse_fn);
        handler.handle_pre_parse(id);
    }

    #[test]
    fn handler_handles_multiple_success_handlers() {
        let mut handler = Handler::new();
        let id = 1;
        let mut node = Node::new_empty(id, &"Test".to_string());

        handler.add_success_handler(id, success_fn);
        handler.add_success_handler(id, success_fn);
        handler.handle_success(&mut node);

        assert_eq!(node.content, "Success");
    }
}
