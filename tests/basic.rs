
use ordan::*;

#[test]
fn should() {
    /*let x = ((1, 2), (3, 4));
    let mut m = blarg!([ (y, z) ] y, z ; [(w, h)] => (w, h));
    let mut o = m(x);

    let a = o.next();

    //assert_eq!(a, Some((1, 2)));

    let a = o.next();

    //assert_eq!(a, Some((3, 4)));
    */
}

#[test]
fn should_2() {
    #[derive(Debug, PartialEq)]
    struct W(usize);
    let x = (((W(1), W(2)), (W(3), W(4))), ((W(5), W(6)), (W(7), W(8))));
    let m = blarg!([ (y, z) ] y, z ; [(w, h)] w, h; [(l, r)]  => (l, r));

    let a = m.take(100).collect::<Vec<_>>();

    assert_eq!(a, vec![]);
}