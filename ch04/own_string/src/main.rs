fn main() {
    // let s = String::from("hello");

    // let mut s = String::from("hello");

    // s.push_str(", world!"); // push_str() appends a literal to a String

    // println!("{}", s); // This will print `hello, world!`

    // // borrow after move
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    // // clone()
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    // stack-only data: copy
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
