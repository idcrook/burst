#![allow(unused_variables)]
fn main() {
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // appending to String with push_str
    let mut s = String::from("foo");
    s.push_str("bar");
    // appending to String with push_str
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_str takes a string slice
    println!("s2 is {}", s2);
    println!("s1 is {}", s1);
    // appending to String with push (single character)
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);

    // format! macro

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // note: format doesn't take ownership of any of its parameters
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);
}
