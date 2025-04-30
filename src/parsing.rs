
use std::rc::Rc;
use proc_macro::{ TokenTree, Delimiter, Span };
use jlnexus::Parser;
use crate::data::*;

macro_rules! proj {
    ($input:ident, $target:pat, $e:expr, $error:literal) => {
        match $input.get(Error(Span::call_site(), "unexpected end of stream"))? {
            $target => Ok($e),
            t => Err(Error(t.span(), $error)), 
        }
    };
    ($input:ident, $target:pat if $p:expr, $e:expr, $error:literal) => {
        match $input.get(Error(Span::call_site(), "unexpected end of stream"))? { 
            $target if $p => Ok($e),
            t => Err(Error(t.span(), $error)), 
        }
    };
}

pub (crate) fn parse(input : &mut Parser<TokenTree>) -> Result<Pattern, Error> {

    let target : Rc<str> = proj!(input, x @ TokenTree::Ident(_), x.to_string().into(), "expected identifier")?;
    proj!(input, TokenTree::Punct(p) if p.as_char() == '=', (), "expected '='")?;
    proj!(input, TokenTree::Punct(p) if p.as_char() == '>', (), "expected '>'")?;

    let clauses = {
        let first_slice = is_slice(input)?;
        let first_pattern = pattern(input)?;

        let mut rest_clause = input.list(|input| {
            let ns = nexts(input)?;
            proj!(input, TokenTree::Punct(p) if p.as_char() == ';', (), "expected ';'")?;
            let slice = is_slice(input)?;
            let pat = pattern(input)?;
            Ok(Clause { slice, pattern: pat, nexts: ns })
        })?;

        rest_clause.insert(0, Clause { slice: first_slice, pattern: first_pattern, nexts: vec![] });

        rest_clause
    };

    proj!(input, TokenTree::Punct(p) if p.as_char() == '=', (), "expected '='")?;
    proj!(input, TokenTree::Punct(p) if p.as_char() == '>', (), "expected '>'")?;

    let return_expr : Rc<str> = input.list(|input| 
        Ok(input.get(Error(Span::call_site(), "unexpected end of stream"))?.to_string()))?.join("").into();

    Ok(Pattern { target, clauses, return_expr })
}

fn pattern(input : &mut Parser<TokenTree>) -> Result<Rc<str>, Error> {
    proj!(input, TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket, g.stream().to_string().into(), "expected '[]' pattern")
}

fn nexts(input : &mut Parser<TokenTree>) -> Result<Vec<Rc<str>>, Error> {
    let first : Rc<str> = proj!(input, x @ TokenTree::Ident(_), x.to_string(), "expected identifier")?.into();
    let mut rest : Vec<Rc<str>> = input.list(|input| {
        proj!(input, TokenTree::Punct(p) if p.as_char() == ',', (), "expected ','")?;
        proj!(input, x @ TokenTree::Ident(_), x.to_string().into(), "expected identifier")
    })?;

    rest.insert(0, first);

    Ok(rest)
}

fn is_slice(input : &mut Parser<TokenTree>) -> Result<bool, Error> {
    let result = input.option(|input| {
        let value = proj!(input, x @ TokenTree::Ident(_), x.to_string(), "expected identifier")?;
        if value == "slice" {
            Ok(Some(()))
        }
        else {
            Ok(None)
        }
    })?;

    Ok(result.is_some())
}