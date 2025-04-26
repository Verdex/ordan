
use std::rc::Rc;
use crate::data::*;

pub (crate) fn gen_pattern(mut pattern : Pattern) -> Rc<str> {

    let mut ret = R::ReturnExpr(pattern.return_expr);

    while let Some(clause) = pattern.clauses.pop() {
        if clause.nexts.len() == 0 {
            ret = R::Match { input: "input".into(), pattern: Rc::clone(&clause.pattern), expr: Box::new(ret) };
        }
        else {
            ret = R::SyntaxList(clause.nexts.into_iter()
                .map(|next| R::Match { input: next, pattern: Rc::clone(&clause.pattern), expr: Box::new(ret.clone()) })
                .collect::<Vec<_>>())
        }
    }

    let target = pattern.target;

    // Note:  Compute this first so that ID will have the correct value.
    let match_statement = r_to_str(&ret);

    let total_ids = unsafe { ID };

    let guards = (0..total_ids).map(|x| format!("let mut x_{x} = true;")).collect::<Vec<_>>().join("");

    format!(
        
        "{{

        use std::borrow::Borrow;

        let input = {target}.borrow();
        
        {guards}

        std::iter::from_fn( move || {{ {match_statement} \n return None; }} )
        
        }}", 


        ).into()
}

enum R {
    Match { input : Rc<str>, pattern : Rc<str>, expr : Box<R> },
    ReturnExpr(Rc<str>),
    SyntaxList(Vec<R>),
}

impl Clone for R {
    fn clone(&self) -> Self {
        match self {
            R::Match { input, pattern, expr } => R::Match { 
                input: Rc::clone(input), 
                pattern: Rc::clone(pattern), 
                expr: expr.clone(),
            },
            R::ReturnExpr(s) => R::ReturnExpr(Rc::clone(s)),
            R::SyntaxList(l) => R::SyntaxList(l.clone()),
        }
    }
}

static mut ID : usize = 0;

fn r_to_str(input : &R) -> Rc<str> {
    match input {
        R::Match { input, pattern, expr } => gen_match(&input, &pattern, &r_to_str(expr)),
        R::ReturnExpr(s) => {
            let id = unsafe {
                let t = ID;
                ID += 1;
                t
            };
            format!("if x_{id} {{ x_{id} = false; return Some({}); }}", s).into()
        }
        R::SyntaxList(l) => {
            let nexts = l.into_iter()
                         .map(|x| r_to_str(x))
                         .collect::<Vec<_>>()
                         .join("\n");

            nexts.into()
        },
    }
}

fn gen_match(input : &str, pattern : &str, expr : &str) -> Rc<str> {
    format!("match {} {{
        {} => {{ {} }}, 
        _ => {{ }},
    }}", input, pattern, expr).into()
}
