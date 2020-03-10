#![allow(unused_variables)]
use std::fs::File;

fn main() {
    // let f: u32 = File::open("hello.txt"); // use bogus type to have compiler inform on the return type
    // expected `u32`, found enum `std::result::Result`
    // -> io::Result<File>
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
        // thread 'main' panicked at 'Problem opening the file: Os { code: 2,
        // kind: NotFound, message: "No such file or directory" }',
        // src/main.rs:12:23
    };
}
