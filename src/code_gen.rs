
use std::rc::Rc;
use crate::data::*;


pub (crate) fn gen_pattern(mut pattern : Pattern) -> Rc<str> {

    let mut ret : Rc<str> = pattern.return_expr;
    while let Some(clause) = pattern.clauses.pop() {
    }
    //let v = gen_match("input", &pattern.clauses[0].pattern, &pattern.return_expr);

    format!("|input| {}", ret).into()
}

fn gen_match(input : &str, pattern : &str, expr : &str ) -> Rc<str> {
    format!("match {} {{
        {} => {{ {} }}, 
        _ => {{ }},
    }}", input, pattern, expr).into()
}
