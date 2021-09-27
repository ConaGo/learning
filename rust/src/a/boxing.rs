#[cfg(test)]
//box allocates on the heap and takes care of deallocation
//box is first initialized by Box::new
//
#[test]
fn test_boxing() {
    let variable = 7;
    let variable_box: Box<i32>;
    let mut mutable_ref: &i32 = &variable;

    assert_eq!(variable, 7);
    assert_eq!(*mutable_ref, 7);

    variable_box = Box::new(variable + 2);
    mutable_ref = &*variable_box; //trick to avoid type incompability

    assert_eq!(variable, 7);
    assert_eq!(*mutable_ref, 9);
    assert_eq!(*variable_box, 9);
}

#[test]
fn test_boxing_mutable() {
    let variable = 7;
    let mut mutable_box: Box<i32>;
    let reference: &i32 = &variable;

    assert_eq!(variable, 7);
    assert_eq!(*reference, 7);

    mutable_box = Box::new(variable + 2);

    assert_eq!(*mutable_box, 9);

    //here the value 9 on the heap gets deallocated
    mutable_box = Box::new(*reference);

    assert_eq!(*mutable_box, 7);
    assert_eq!(*reference, 7);
}
