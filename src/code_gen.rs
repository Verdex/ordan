
use std::rc::Rc;
use crate::data::*;

pub (crate) fn gen_pattern(pattern : Pattern) -> Rc<str> {
    gen_match("x", &pattern.patterns[0], &pattern.return_expr)
}

fn gen_match(input : &str, pattern : &str, expr : &str ) -> Rc<str> {
    format!("match {} {{
        {} => {{ {} }}, 
        _ => {{ }},
    }}", input, pattern, expr).into()
}