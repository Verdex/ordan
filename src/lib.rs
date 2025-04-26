
mod data;
mod parsing;
mod code_gen;

use proc_macro::*;
use jlnexus::Parser;

#[proc_macro]
pub fn s_pattern( input : TokenStream ) -> TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();
    let mut parser = Parser::new(&input);
    let result = parsing::parse(&mut parser).unwrap();

    code_gen::gen_pattern(result).parse().unwrap()
}