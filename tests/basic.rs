
use ordan::*;

#[test]
fn should() {
    blarg!([ (x, y) ] => {outputy + 1});
    assert!(false);
}