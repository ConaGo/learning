#![allow(dead_code)]
// passing arguments to functions

//passing by value
fn main() {}
//variable gets copied
//function operates on copied value

fn pass_by_value() -> (f64, f64) {
    fn double(mut x: f64) -> f64 {
        x *= 2.; //variable is mutable only in this scope
        x
    }
    let x = 4.;
    // x *= 2.; //and not in this scope
    (x, double(x))
    // 8 4
}
#[cfg(test)]
#[test]
fn test_pass_by_value() {
    assert_eq!(pass_by_value(), (4., 8.));
}

//passing by reference

//The "&" symbol means “the (memory) address of the object ...”, and
//the "*" symbol means “the object that is present at the (memory) address ...”.

fn double_negatives(a: &mut [i32; 10]) {
    for i in 0..10 {
        if a[i] < 0 {
            //compiler infers dereferencing
            (*a)[i] *= 2; //so a[i] is equal to (*a)[i]
        }
    }
}
#[cfg(test)]
#[test]
fn test_pass_by_reference() {
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double_negatives(&mut arr);
    assert_eq!(arr, [5, -8, 9, 0, -14, -2, 3, 5, 3, 1]);
}

fn mutable_refs() {
    let mut a: i32 = 10;
    let mut b: i32 = 20;
    let mut p: &mut i32 = &mut a;
    print!("{} ", *p);
    *p += 1;
    print!("{} ", *p);
    p = &mut b;
    print!("{} ", *p);
    *p += 1;
    print!("{} ", *p);
    //prints 10 11 20 21
}
