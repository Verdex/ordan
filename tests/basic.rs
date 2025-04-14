
use ordan::*;

#[test]
fn should() {
    let x = (1, 2);
    let mut o = 0;
    let mut m = blarg!([ (y, z) ] y, z ; [(w, h)] => o = y + z);
    m(x);

    assert_eq!(o, 3);
}