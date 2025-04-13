
use std::rc::Rc;
use proc_macro::{ TokenTree, Delimiter };
use jlnexus::Parser;
use crate::data::*;

macro_rules! proj {
    ($input:ident, $target:pat, $e:expr) => {
        match $input.get(())? { // TODO error
            $target => Ok($e),
            _t => Err(()), // TODO error
        }
    };
    ($input:ident, $target:pat if $p:expr, $e:expr) => {
        match $input.get(())? { // TODO error
            $target if $p => Ok($e),
            _t => Err(()), // TODO error
        }
    };
}

pub (crate) fn parse(input : &mut Parser<TokenTree>) -> Result<Pattern, ()> {
    let clauses = {
        let first_clause = clause(input)?;

        let mut rest_clause = input.list(|input| {
            proj!(input, TokenTree::Punct(p) if p.as_char() == ';', ())?;
            clause(input)
        })?;

        rest_clause.insert(0, first_clause);

        rest_clause
    };

    proj!(input, TokenTree::Punct(p) if p.as_char() == '=', ())?;
    proj!(input, TokenTree::Punct(p) if p.as_char() == '>', ())?;

    let return_expr : Rc<str> = input.list(|input| Ok(input.get(())?.to_string()))?.join("").into();   // TODO get error

    Ok(Pattern { clauses, return_expr })
}

fn clause(input : &mut Parser<TokenTree>) -> Result<Clause, ()> {
    let pattern = proj!(input, TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket, g.stream().to_string().into())?;
    match input.option(|input| nexts(input))? {
        Some(nexts) => Ok(Clause { pattern, nexts }),
        None => Ok(Clause { pattern, nexts: vec![] }),
    }
}

fn nexts(input : &mut Parser<TokenTree>) -> Result<Vec<Rc<str>>, ()> {
    let first : Rc<str> = proj!(input, x @ TokenTree::Ident(_), x.to_string())?.into();
    let mut rest : Vec<Rc<str>> = input.list(|input| {
        proj!(input, TokenTree::Punct(p) if p.as_char() == ',', ())?;
        proj!(input, x @ TokenTree::Ident(_), x.to_string().into())
    })?;

    rest.insert(0, first);

    Ok(rest)
}