
use std::rc::Rc;
use crate::data::*;

pub (crate) fn gen_pattern(mut pattern : Pattern) -> Rc<str> {

    let mut ret = R::ReturnExpr(pattern.return_expr);
    while let Some(clause) = pattern.clauses.pop() {
        if clause.nexts.len() == 0 {
            ret = R::Match { input: "input".into(), pattern: Rc::clone(&clause.pattern), expr: Rc::new(ret) };
        }
        else {
            ret = R::SyntaxList(clause.nexts.into_iter()
                .map(|next| R::Match { input: next, pattern: Rc::clone(&clause.pattern), expr: Rc::new(ret.clone()) })
                .collect::<Vec<_>>())
        }
    }

    r_to_str(&ret)
}

enum R {
    Match { input : Rc<str>, pattern : Rc<str>, expr : Rc<R> },
    ReturnExpr(Rc<str>),
    SyntaxList(Vec<R>),
}

impl Clone for R {
    fn clone(&self) -> Self {
        match self {
            R::Match { input, pattern, expr } => R::Match { 
                input: Rc::clone(input), 
                pattern: Rc::clone(pattern), 
                expr: Rc::clone(expr)
            },
            R::ReturnExpr(s) => R::ReturnExpr(Rc::clone(s)),
            R::SyntaxList(l) => R::SyntaxList(l.clone()),
        }
    }
}

fn r_to_str(input : &R) -> Rc<str> {
    match input {
        R::Match { input, pattern, expr } => gen_match(&input, &pattern, &r_to_str(expr)),
        R::ReturnExpr(s) => Rc::clone(s),
        R::SyntaxList(l) => l.into_iter().map(|x| r_to_str(x)).collect::<Vec<_>>().join("\n").into(),
    }
}

fn gen_match(input : &str, pattern : &str, expr : &str) -> Rc<str> {
    format!("match {} {{
        {} => {{ {} }}, 
        _ => {{ }},
    }}", input, pattern, expr).into()
}
