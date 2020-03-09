use std::vec::Vec;
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let v: Vec<i32> = Vec::new();

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
