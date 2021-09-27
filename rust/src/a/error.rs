#![allow(dead_code)]
//prints 33, 22, 11, #, #
fn safe_pop() {
    let mut v = vec![11, 22, 33];
    for _ in 0..5 {
        let item: Option<i32> = v.pop();
        match item {
            Some(number) => print!("{}, ", number),
            None => print!("#, "),
        }
    }
}

//Error Handling
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err(format!("Divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

#[cfg(test)]
#[test]
fn test_divide() {
    let t = divide(8., 2.);
    let f = divide(8., 0.);
    assert_eq!(t, Ok(4.));
    assert_eq!(f, Err(format!("Divide by zero")));
    assert!(t.is_ok());
    assert!(f.is_err());
    assert_eq!(t.unwrap(), 4.);
    assert_eq!(f.unwrap_err(), "Divide by zero");
}

fn show_divide(num: f64, den: f64) -> &'static str {
    match divide(num, den) {
        Ok(_) => "OK",
        Err(_) => "ERR",
    }
}
#[test]
fn test_show_divide() {
    let t = show_divide(8., 2.);
    let f = show_divide(8., 0.);
    assert_eq!(t, "OK");
    assert_eq!(f, "ERR");
}
