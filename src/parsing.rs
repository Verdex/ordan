
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

pub fn parse(input : &mut Parser<TokenTree>) -> Result<Pattern, ()> {
    let patterns = {
        let first_pattern = pattern(input)?;

        let mut rest_pattern = input.list(|input| {
            proj!(input, TokenTree::Punct(p) if p.as_char() == ';', ())?;
            pattern(input)
        })?;

        rest_pattern.insert(0, first_pattern);

        rest_pattern
    };

    proj!(input, TokenTree::Punct(p) if p.as_char() == '=', ())?;
    proj!(input, TokenTree::Punct(p) if p.as_char() == '>', ())?;

    let return_expr : Rc<str> = input.list(|input| Ok(input.get(())?.to_string()))?.join("").into();   // TODO get error

    Ok(Pattern { patterns, return_expr })
}

fn pattern(input : &mut Parser<TokenTree>) -> Result<Rc<str>, ()> {
    proj!(input, TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket, g.stream().to_string().into())
}