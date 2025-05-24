
use std::rc::Rc;
use proc_macro::Span;

use jlnexus::JlnError;

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

impl JlnError for Error {
    fn is_fatal(&self) -> bool { false }
    fn eof() -> Self { Error(Span::call_site(), "unexpected end of stream") }
    fn aggregate(_errors : Vec<Self>) -> Self { JlnError::eof() }
}