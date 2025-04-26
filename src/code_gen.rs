
use std::rc::Rc;
use crate::data::*;

pub (crate) fn gen_pattern(mut pattern : Pattern) -> Rc<str> {

    let mut ret = R::ReturnExpr(0, pattern.return_expr);

    while let Some(clause) = pattern.clauses.pop() {
        if clause.nexts.len() == 0 {
            ret = R::Match { input: "input".into(), pattern: Rc::clone(&clause.pattern), expr: Box::new(ret) };
        }
        else {
            ret = R::SyntaxList(clause.nexts.into_iter()
                .map(|next| R::Match { input: next, pattern: Rc::clone(&clause.pattern), expr: Box::new(ret.spawn()) })
                .collect::<Vec<_>>())
        }
    }

    // TODO input : &_ ?
    let w :Rc<str>= 
        format!(
            
            "|input| {{
            
            std::iter::from_fn( move || {{ {} \n return None; }} )
            
            }}", 
    
            r_to_str(&ret)).into();

    println!("{}", w);


    w
}

enum R {
    Match { input : Rc<str>, pattern : Rc<str>, expr : Box<R> },
    ReturnExpr(usize, Rc<str>),
    SyntaxList(Vec<R>),
}

impl R {
    pub fn spawn(&self) -> Self {
        match self {
            R::Match { input, pattern, expr } => R::Match { 
                input: Rc::clone(input), 
                pattern: Rc::clone(pattern), 
                expr: Box::new(expr.spawn()),
            },
            R::ReturnExpr(id, s) => R::ReturnExpr(*id, Rc::clone(s)),
            R::SyntaxList(l) => R::SyntaxList(l.iter().map(|x| x.spawn()).collect()),
        }
    }
}

fn r_to_str(input : &R) -> Rc<str> {
    match input {
        R::Match { input, pattern, expr } => gen_match(&input, &pattern, &r_to_str(expr)),
        R::ReturnExpr(_, s) => format!("return Some({})", s).into(),
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
