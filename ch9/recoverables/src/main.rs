#![allow(unused_variables)]
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    println!("{:?}", f);

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_operator_question_mark() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    // error values that have the ? operator called on them go through the from
    // function, defined in the From trait in the standard library, which is
    // used to convert errors from one type into another.
    Ok(s)
}

fn read_username_from_file_chaining() -> Result<String, io::Error> {
    //let mut f = File::open("hello.text")?;
    let mut s = String::new();

    File::open("hello.text")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_fn_read_to_string() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// fn main() {
//     let uname = read_username_from_file();
//     let uname = match uname {
//         Ok(s) => s,
//         Err(e) => panic!(e),
//     };
//     println!("{}", uname);
// }

// `Box<dyn Error>` is a trait object
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
