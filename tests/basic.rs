
use ordan::*;

#[test]
fn should() {
    let x = (1, 2);
    let mut o = 0;
    blarg!([ (y, z) ] => o = y + z);

    assert_eq!(o, 3);
}