#![allow(unused_variables)]
fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    // rust infers type from this data
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // panics // let does_not_exist = &v[100];
    // returns None // let does_not_exist = v.get(100);

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6); // mutable and immutable borrow not allowed
    // println!("The first element is: {}", first);

    // immutable ref iteration
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // mutable ref iteration
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // enum Type can have heterogeneous variants
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
} // <- v goes out of scope and is freed here
