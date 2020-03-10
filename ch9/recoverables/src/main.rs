#![allow(unused_variables)]
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // // shortcut to panic if Error
    // let f = File::open("hello.txt").unwrap();
    //thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:
    // Os { code: 2, kind: NotFound, message: "No such file or directory" }',
    // src/libcore/result.rs:1188:5

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

// // version using closures
// fn main() {
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }
