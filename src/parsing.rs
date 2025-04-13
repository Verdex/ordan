
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

pub fn parse(input : &mut Parser<TokenTree>) -> Result<Ast, ()> {
    proj!(semi, TokenTree::Punct(p) if p.as_char() == ';', ());
    proj!(comma, TokenTree::Punct(p) if p.as_char() == ',', ());
    proj!(pattern, TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket, g.stream().to_string());

    todo!()
}
