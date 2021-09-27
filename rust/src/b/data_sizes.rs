#[cfg(test)]
#[test]

fn test_size() {
    assert_eq!(std::mem::size_of::<i32>(), 4);
    assert_eq!(std::mem::size_of_val(&12), 4)
}
