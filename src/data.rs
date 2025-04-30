
use std::rc::Rc;
use proc_macro::Span;

pub (crate) struct Clause {
    pub (crate) slice : bool,
    pub (crate) pattern : Rc<str>,
    pub (crate) nexts : Vec<Rc<str>>,
}

pub (crate) struct Pattern {
    pub (crate) target : Rc<str>,
    pub (crate) clauses : Vec<Clause>,
    pub (crate) return_expr : Rc<str>,
}

pub (crate) struct Error(pub (crate) Span, pub (crate) &'static str);