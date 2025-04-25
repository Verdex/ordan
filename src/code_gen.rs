
use std::rc::Rc;
use crate::data::*;

pub (crate) fn gen_pattern(mut pattern : Pattern) -> Rc<str> {

    let mut gates : Vec<Rc<str>> = vec![];
    let mut id = 0;
    let mut ret = R::ReturnExpr(pattern.return_expr);

    while let Some(clause) = pattern.clauses.pop() {
        if clause.nexts.len() == 0 {
            ret = R::Match { input: "input".into(), pattern: Rc::clone(&clause.pattern), expr: Rc::new(ret) };
        }
        else {
            let x : Rc<str> = format!("x_{}", id).into();
            gates.push(Rc::clone(&x));
            id += 1;
            ret = R::SyntaxList(x, clause.nexts.into_iter()
                .map(|next| R::Match { input: next, pattern: Rc::clone(&clause.pattern), expr: Rc::new(ret.clone()) })
                .collect::<Vec<_>>())
        }
    }

    // TODO input : &_ ?
    let w :Rc<str>= 
        format!(
            
            "|input| {{
            
            {}
            
            std::iter::from_fn( move || {{ {} \n return None; }} )
            
            }}", 
    
            gates.into_iter().map(|gate| format!("let mut {} = 0;\n", gate)).collect::<Vec<_>>().join(""),
    
            r_to_str(&ret)).into();

    println!("{}", w);

    panic!();

    w
}

enum R {
    Match { input : Rc<str>, pattern : Rc<str>, expr : Rc<R> },
    ReturnExpr(Rc<str>),
    SyntaxList(Rc<str>, Vec<R>),
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
            R::SyntaxList(id, l) => R::SyntaxList(Rc::clone(id), l.clone()),
        }
    }
}

fn r_to_str(input : &R) -> Rc<str> {
    match input {
        R::Match { input, pattern, expr } => gen_match(&input, &pattern, &r_to_str(expr)),
        R::ReturnExpr(s) => format!("return Some({})", s).into(),
        R::SyntaxList(id, l) => {
            let c = l.len();
            let nexts = l.into_iter()
                         .enumerate()
                         .map(|(i, x)| format!("if {id} == {i} {{ {id} = ({id} + 1) % {c}; {} }} ", r_to_str(x)))
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
