
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
    let x = (((1, 2), (3, 4)), ((5, 6), (7, 8)));
    let m = blarg!([ (y, z) ] y, z ; [(w, h)] w, h; [(l, r)]  => (l, r));
    let o = m(x);

    let a = o.take(100).collect::<Vec<_>>();

    assert_eq!(a, vec![]);
}