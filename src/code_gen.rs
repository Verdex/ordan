
use std::rc::Rc;
use crate::data::*;

pub (crate) fn gen_pattern(pattern : Pattern) -> Rc<str> {

    let v = gen_match("input", &pattern.clauses[0].pattern, &pattern.return_expr);

    format!("|input| {}", v).into()
}

fn gen_match(input : &str, pattern : &str, expr : &str ) -> Rc<str> {
    format!("match {} {{
        {} => {{ {} }}, 
        _ => {{ }},
    }}", input, pattern, expr).into()
}