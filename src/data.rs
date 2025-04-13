
use std::rc::Rc;

pub (crate) struct Clause {
    pub (crate) pattern : Rc<str>,
    pub (crate) nexts : Vec<Rc<str>>,
}

pub (crate) struct Pattern {
    pub (crate) clauses : Vec<Clause>,
    pub (crate) return_expr : Rc<str>,
}