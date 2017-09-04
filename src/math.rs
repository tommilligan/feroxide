extern crate num;
use self::num::*;


/// Calculate Greatest Common Divisor (GCD), using Euclides' algorithm
pub fn gcd<T: Integer + FromPrimitive + Copy + PartialOrd>(x: T, y: T) -> T {
    // Store the highest in a, the lowest in b
    let (mut a, mut b) =
        if x > y {
            (x, y)
        } else {
            (y, x)
        };

    while b != T::from_u32(0).unwrap() {
        let rem = a % b;
        a = b;
        b = rem;
    }

    a
}


pub fn diff<T: Num + Signed>(a: T, b: T) -> T {
    T::from((a - b).abs())
}
