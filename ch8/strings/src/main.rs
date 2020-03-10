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

    // indexing into strings
    let s1 = String::from("hello");
    // let h = s1[0]; // error[E0277]: the type `std::string::String` cannot be indexed by `{integer}`
    let len = String::from("Hola").len();
    println!("string Hola has .len() {}", len);
    let len = String::from("Здравствуйте").len();
    println!("string Здравствуйте has .len() {}", len); // 24, not 12

    // String is a wrapper over a Vec<u8> and UTF-8 encoded. There are three
    // relevant ways to consider strings from Rust's perspective: bytes,
    // (Unicode) scalar values, and grapheme clusters

    // these all represent the same Hindi word: “नमस्ते” written in the Devanagari script
    let u8_bytes = [
        224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135,
    ];
    let unicode_scalars = ['न', 'म', 'स', '्', 'त', 'े'];
    let grapheme_clusters = ["न", "म", "स्", "ते"];
    // TL:;DR: Rust doesn’t allow us to index into a String

    // slicing strings
    let hello = "Здравствуйте";
    //let s = &hello[0..1]; // causes runtime panic
    let s = &hello[0..4]; // first four bytes of string
    println!("s is {}", s); // s is Зд

    //let s = hello.get(0..1);
    if let Some(s) = hello.get(0..1) {
        println!("s is {}", s);
    } else {
        println!(
            "index 0 character boundary? {:?}",
            hello.is_char_boundary(0)
        );
        println!(
            "index 1 character boundary? {:?}",
            hello.is_char_boundary(1)
        );
        println!(
            "index 2 character boundary? {:?}",
            hello.is_char_boundary(2)
        );
    }

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }
    println!("");

    for gc in "नमस्ते".char_indices() {
        println!("{:?}", gc);
    }
}
