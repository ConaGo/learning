#[cfg(test)]
#[test]
//normal function declaration
//dynamic environment does not get captured
//consts are accessible
fn test_order_des() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    use std::cmp::Ordering;
    fn desc(a: &i32, b: &i32) -> Ordering {
        if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
    arr.sort_by(desc);
    assert_eq!(arr, [45, 12, 10, 8, 7, 4, 1, 0]);
}
//same function with closure
//small anonymous function
//can acces dynamic environment
//immediately invoked or assigned to variable
// pipe symbol | instead of ()
#[test]
fn test_sort_des_closure() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    use std::cmp::Ordering;
    let desc = |a: &i32, b: &i32| -> Ordering {
        if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    };
    arr.sort_by(desc);
    assert_eq!(arr, [45, 12, 10, 8, 7, 4, 1, 0]);
}

#[test]
fn test_shorthand() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    arr.sort_by(|a, b| b.cmp(a));
    assert_eq!(arr, [45, 12, 10, 8, 7, 4, 1, 0]);
}
