use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap(); // unwrap will cause a panic upon an error
            thread::sleep(Duration::from_secs(1));
        }
    });

    // can treat rx like an iterator
    for received in rx {
        println!("Got: {}", received);
    }
}
