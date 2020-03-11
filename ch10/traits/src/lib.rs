#![allow(unused_variables)]
use std::fmt::Debug;
use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// // can call any method on item that implements specified trait (Summary in this
// // example)
// pub fn notify(item: impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// longer form of the above, called "trait bound" syntax
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

//pub fn notify2(item1: impl Summary, item2: impl Summary) {}
pub fn notify2<T: Summary>(item1: T, item2: T) {} // forces both parameters to
                                                  // have the same type, not
                                                  // just implement the trait

// specify multiple trait bounds with + Syntax
//pub fn notify_d(item: impl Summary + Display) {}
pub fn notify_d<T: Summary + Display>(item: T) {}
//pub fn notify_d<T: Summary, D: Display>(item: T, formatter: D) {}

// pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
//     0
// }

// where clauses for cleaner trait bounds
pub fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}
