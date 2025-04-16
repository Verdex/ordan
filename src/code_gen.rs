
use std::rc::Rc;
use crate::data::*;


pub (crate) fn gen_pattern(mut pattern : Pattern) -> Rc<str> {

    let mut ret : Rc<str> = pattern.return_expr;
    while let Some(clause) = pattern.clauses.pop() {
        if clause.nexts.len() == 0 {
            ret = gen_match("input", &clause.pattern, &ret);
        }
        else {
            ret = clause.nexts.into_iter()
                .map(|next| gen_match(&next, &clause.pattern, &ret))
                .collect::<Vec<_>>()
                .join("\n").into();
        }
    }

    format!("|input| {}", ret).into()
}

fn gen_match(input : &str, pattern : &str, expr : &str) -> Rc<str> {
    format!("match {} {{
        {} => {{ {} }}, 
        _ => {{ }},
    }}", input, pattern, expr).into()
}
