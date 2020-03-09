mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// // bring a module into scope
// use crate::front_of_house::hosting;

// unidiomatic since it is local
// use crate::front_of_house::hosting::add_to_waitlist;

// re-exporting with pub use
pub use crate::front_of_house::hosting;
//  external code can now call the add_to_waitlist function using
//  hosting::add_to_waitlist

pub fn eat_at_restaurant() {
    // use a relative path
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
fn main() {}
