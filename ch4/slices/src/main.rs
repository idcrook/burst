fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];

    println!("{}", slice);

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
    let slice = &s[0..len];
    let slice = &s[..];
    println!("{}", slice);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
