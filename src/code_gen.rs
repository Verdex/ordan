
use std::rc::Rc;
use crate::data::*;

const TAG : &str = "ordan0";

pub (crate) fn gen_pattern(mut pattern : Pattern) -> Rc<str> {

    let mut ret = R::ReturnExpr(pattern.return_expr);

    let mut initial_slice = false;

    while let Some(clause) = pattern.clauses.pop() {
        if clause.nexts.len() == 0 {
            initial_slice = clause.slice;
            ret = R::Match { 
                input: format!("input_{TAG}").into(), 
                pattern: Rc::clone(&clause.pattern), 
                expr: Box::new(ret),
                slice: clause.slice,
            };
        }
        else {
            ret = R::SyntaxList(clause.nexts.into_iter()
                .map(|next| R::Match { 
                    input: next, 
                    pattern: Rc::clone(&clause.pattern), 
                    expr: Box::new(ret.clone()),
                    slice: clause.slice,
                })
                .collect::<Vec<_>>())
        }
    }

    let target = pattern.target;

    // Note:  Compute this first so that ID will have the correct value.
    let match_statement = r_to_str(&ret);

    let total_ids = unsafe { ID };

    let guards = (0..total_ids).map(|x| format!("let mut guard_{TAG}_{x} = true;")).collect::<Vec<_>>().join("");

    let maybe_borrow = if initial_slice {
        ""
    }
    else {
        ".borrow()"
    };

    format!(
        
        "{{

        use std::borrow::Borrow;

        let input_{TAG} = {target}{maybe_borrow};
        
        {guards}

        std::iter::from_fn( move || {{ {match_statement} \n return None; }} )
        
        }}", 


        ).into()
}

enum R {
    Match { input : Rc<str>, pattern : Rc<str>, expr : Box<R>, slice : bool },
    ReturnExpr(Rc<str>),
    SyntaxList(Vec<R>),
}

impl Clone for R {
    fn clone(&self) -> Self {
        match self {
            R::Match { input, pattern, expr, slice } => R::Match { 
                input: Rc::clone(input), 
                pattern: Rc::clone(pattern), 
                expr: expr.clone(),
                slice: *slice,
            },
            R::ReturnExpr(s) => R::ReturnExpr(Rc::clone(s)),
            R::SyntaxList(l) => R::SyntaxList(l.clone()),
        }
    }
}

static mut ID : usize = 0;

fn r_to_str(input : &R) -> Rc<str> {
    match input {
        R::Match { input, pattern, expr, slice: false } => {
            format!("match {input}.borrow() {{
                {pattern} => {{ {} }}, 
                _ => {{ }},
            }}", r_to_str(expr)).into()
        }, 
        R::Match { input, pattern, expr, slice: true } => {
            format!("match &{input}[..] {{
                {pattern} => {{ {} }}, 
                _ => {{ }},
            }}", r_to_str(expr)).into()
        }, 
        R::ReturnExpr(s) => {
            let id = unsafe {
                let t = ID;
                ID += 1;
                t
            };
            format!("if guard_{TAG}_{id} {{ guard_{TAG}_{id} = false; return Some({}); }}", s).into()
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
