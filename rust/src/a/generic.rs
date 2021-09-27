#![allow(dead_code)]
fn generic<T>(ch: char, num1: T, num2: T) -> T {
    if ch == 'a' {
        num1
    } else {
        num2
    }
}
#[cfg(test)]
#[test]
fn test_generic() {
    let a: i16 = generic::<i16>('a', 37, 41);
    assert_eq!(a, 37);
    let b: f64 = generic('b', 37.2, 41.1); //type gets inferred
    assert_eq!(b, 41.1);
}

fn generic_struc() {
    struct S<T1, T2> {
        c: char,
        n1: T1,
        n2: T1,
        n3: T2,
    }
    let _s = S {
        c: 'a',
        n1: 34,
        n2: 782,
        n3: 0.02,
    };
    //explicit
    let _s = S::<u16, f32> {
        c: 'a',
        n1: 34,
        n2: 782,
        n3: 0.02,
    };
    //tuple-struct
    struct SE<T1, T2>(char, T1, T1, T2);
    let _se = SE('a', 34, 782, 0.02);
}

//generic enums
fn generic_enum() {
    enum Result1<SuccessCode, FailureCode> {
        Success(SuccessCode),
        Failure(FailureCode, char),
        Uncertainty,
    }
    let mut _res = Result1::Success::<u32, u16>(12u32);
    _res = Result1::Uncertainty;
    _res = Result1::Failure(0u16, 'd');
}
