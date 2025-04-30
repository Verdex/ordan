
use ordan::*;

#[test]
fn should_get_items() {
    let input = (((1, 2), (3, 4)), ((5, 6), (7, 8)));
    let output = s_pattern!(input => [ (y, z) ] y, z ; [(w, h)] w, h; [(l, r)]  => (l, r))
        .map(|(a, b)| (*a, *b))
        .collect::<Vec<_>>();
    assert_eq!(output, vec![(1, 2), (3, 4), (5, 6), (7, 8)]);
}

#[test]
fn should_use_previous_variables() {
    let input = (((1, 2), (3, 4)), ((5, 6), (7, 8)));
    let output = s_pattern!(input => [ (y, z) ] y, z ; [(w, h)] w, h; [(l, r)]  => (y.0.0, w.1, l, r))
        .map(|(a, b, c, d)| (a, b, *c, *d))
        .collect::<Vec<_>>();
    assert_eq!(output, vec![(1, 2, 1, 2), (1, 2, 3, 4), (1, 6, 5, 6), (1, 6, 7, 8)]);
}

#[test]
fn should_handle_no_copy_data() {
    #[derive(Debug, PartialEq)]
    struct W(usize);

    let input = (((W(1), W(2)), (W(3), W(4))), ((W(5), W(6)), (W(7), W(8))));
    let output = s_pattern!(input => [ (y, z) ] y, z ; [(w, h)] w, h; [(l, r)]  => (l, r)).collect::<Vec<_>>();
    assert_eq!(output, vec![(&W(1), &W(2)), (&W(3), &W(4)), (&W(5), &W(6)), (&W(7), &W(8))]);
}

#[test]
fn should_allow_if_filter() {
    let input = (((1, 2), (3, 4)), ((5, 6), (7, 8)));
    let output = s_pattern!(input => [ (y, z) ] y, z ; [(w, h) if w.0 < 5] w, h; [(l, r)]  => (l, r))
        .map(|(a, b)| (*a, *b))
        .collect::<Vec<_>>();
    assert_eq!(output, vec![(1, 2), (3, 4)]);
}

#[test]
fn should_allow_pattern_filter() {
    let input = (((1, 2), (0, 4)), ((1, 6), (0, 8)));
    let output = s_pattern!(input => [ (y, z) ] y, z ; [(w, h)] w, h; [(1, r)]  => r)
        .map(|x| *x)
        .collect::<Vec<_>>();
    assert_eq!(output, vec![2, 6]);
}

#[test]
fn should_handle_enum() {
    enum T {
        N(Box<T>, Box<T>),
        L(usize),
    }

    let input = T::N(Box::new(T::N(Box::new(T::L(1)), Box::new(T::L(2)))), Box::new(T::L(0)));
    let output = s_pattern!(input => [ T::N(a, b) ] a, b; [ T::L(x) ] => *x).collect::<Vec<_>>();
    assert_eq!(output, vec![0]);
}

#[test]
fn should_handle_vec() {
    let input = vec![ vec![1, 2], vec![3, 4] ];
    let output = s_pattern!(input => slice [ [a, b] ] a, b; slice [ [c, d] ] => (*c, *d)).collect::<Vec<_>>();
    assert_eq!(output, vec![(1, 2), (3, 4)]);
}

#[test]
fn should_handle_inner_vec() {
    struct Item(Vec<usize>);

    let input = Item(vec![1, 2, 3]);
    let output = s_pattern!(input => [ Item(x) ] x; slice [ [c, d, ..] ] => (*c, *d)).collect::<Vec<_>>();
    assert_eq!(output, vec![(1, 2)]);
}