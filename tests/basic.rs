
use ordan::*;

#[test]
fn should() {
    let x = ((1, 2), (3, 4));
    let mut o = 0;
    let mut m = blarg!([ (y, z) ] y, z ; [(w, h)] => o += w + h);
    m(x);

    assert_eq!(o, 3);
}