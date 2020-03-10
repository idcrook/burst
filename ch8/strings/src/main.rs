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
}
