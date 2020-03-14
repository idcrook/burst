use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // unwrap will cause a panic upon an error

        // println!("val is {}", val); // oops: borrow of moved value
    });

    // use rx (recv blocks) in the main thread
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
