use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // FIXME: Check length of args vector
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
