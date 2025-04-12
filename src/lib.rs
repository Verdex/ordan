
mod data;
mod parsing;

use proc_macro::*;
use jlnexus::Parser;


#[proc_macro]
pub fn blarg( input : TokenStream ) -> TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();
    let mut parser = Parser::new(&input);
    let result = parsing::parse(&mut parser);
    todo!()
}
/*
[ p1[x] ] x ; [ p2[y] ] => { (x, y) }

match w {
    p1 => {
        match x {
            p2 => {
                yield (x, y)
            },
            _ => { },
        },
    },
    _ => {  },
}

[ p1 ] x, y ; [ p2[w, z] ] => { (w, z) }

match w {
    p1 => {
        match x {
            p2 => {
                yield (w, z)
            }
        },
        match y {
            p2 => {
                yield (w, z)
            }
        },
    },
}
*/

#[cfg(test)]
mod test {
    struct X(usize);

    #[test]
    fn test() {

        let x = X(0);
        let addr_x = &x;

        let mut w = None;

        let i = std::iter::from_fn(move || { 
            if w.is_none() {
                w = Some(std::iter::repeat(addr_x).map(|h| h.0));
            }
            w.as_mut().unwrap().next()
        });

        let z = i.take(5).collect::<Vec<_>>();

        assert_eq!(z, []);
    }
}