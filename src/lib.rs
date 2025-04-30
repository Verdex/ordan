
mod data;
mod parsing;
mod code_gen;

use proc_macro::TokenStream;
use jlnexus::Parser;
use crate::data::Error;

#[proc_macro]
pub fn s_pattern( input : TokenStream ) -> TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();
    let mut parser = Parser::new(&input);

    match parsing::parse(&mut parser) {
        Ok(result) => code_gen::gen_pattern(result).parse().unwrap(),
        Err(error) => gen_compile_error(error),
    }
}

fn gen_compile_error(error : Error) -> TokenStream {
    let mut code = format!("compile_error!(\"{}\");", error.1)
        .parse::<TokenStream>()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();

    let span = error.0;
    for c in &mut code {
        c.set_span(span);
    }
    code.into_iter().collect()
}