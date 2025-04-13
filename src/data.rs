
use std::rc::Rc;

pub (crate) struct Pattern {
    pub (crate) patterns : Vec<Rc<str>>,
    pub (crate) return_expr : Rc<str>,
}