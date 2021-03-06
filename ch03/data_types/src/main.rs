#![allow(unused_variables)]
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // character type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //println!("The value of a[] is: {:?}", a.iter());
    println!("The value of a[] is: {:?}", a);
    let a = [3; 5];
    println!("The value of a[4] is: {}", a[4]);
}
