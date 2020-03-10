#![allow(unused_variables)]
use std::net::IpAddr;

pub struct Guess {
    value: i32,
}

// Listing 9-10: A Guess type that will only continue with values between 1 and 100
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    //let g = Guess::new(190);
    let g = Guess::new(19);
    println!("Guess was {}", g.value());
}
