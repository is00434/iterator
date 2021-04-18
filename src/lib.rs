
#[test]
fn iterator_demo() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let total: u32 = v1.iter().sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_sum2() {
    let v1 = vec![1, 2, 3];

    let total: u32 = v1.iter().map(|x| x + 1).sum();

    assert_eq!(total, 9);
}