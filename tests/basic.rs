
use ordan::*;

#[test]
fn should() {
    let x = ((1, 2), (3, 4));
    let mut m = blarg!([ (y, z) ] y, z ; [(w, h)] => (w, h));
    let mut o = m(x);

    let a = o.next();

    assert_eq!(a, Some((1, 2)));
}