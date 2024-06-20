#[macro_use]
pub mod char;
#[macro_use]
pub mod eof;
// #[macro_use]
// pub mod opt;
// #[macro_use]
// pub mod plus;
#[macro_use]
pub mod seq;
#[macro_use]
pub mod custom;
#[macro_use]
pub mod sor;
// #[macro_use]
// pub mod star;
// #[macro_use]
// pub mod str;
// #[macro_use]
// pub mod ranges;
pub use char::*;
pub use eof::*;
// pub use opt::*;
// pub use plus::*;
// pub use ranges::*;
pub use custom::*;
pub use seq::*;
pub use sor::*;
// pub use star::*;
// pub use str::*;

use std::cell::OnceCell;
use std::rc::Rc;

use crate::Parsable;

#[derive(Clone)]
pub struct Rule<'a> {
    pub rule: Rc<OnceCell<Box<dyn crate::Parsable<'a> + 'a>>>,
    pub id: usize,
    pub name: String,
}

impl<'a> Rule<'a> {
    pub fn new_late_instantiated(name: String) -> Self {
        Self {
            rule: Rc::new(OnceCell::new()),
            id: crate::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            name,
        }
    }

    pub fn new(rule: Box<dyn Parsable<'a> + 'a>, id: usize, name: String) -> Rule<'a> {
        let rule = Rc::new(OnceCell::from(rule));
        Self { rule, id, name }
    }

    pub fn parse(&self, input: &'a str) -> Option<crate::Node<'a>> {
        if let Some(rule) = self.rule.get() {
            rule.parse(input, self.id, &self.name)
        } else {
            panic!("Rule not initialized")
        }
    }

    pub fn get(&self) -> Self {
        self.clone()
    }
}
