use std::fmt::Display;
// fn largest<T>(list: &[T]) -> T {
// fn largest<T: PartialOrd>(list: &[T]) -> T {
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// can conditionally implement a trait for any type that implements another
// trait. For example, the standard library implements the ToString trait on
// any type that implements the Display trait
// impl<T: Display> ToString for T {
//     // --snip--

//     //  	     = note: implementing a foreign trait is only possible if at
//     //  	     least one of the types for which is it implemented is local

//     // = note: only traits defined in the current crate can be implemented for
//     // a type parameter
// }

// This is referred to as a blanket implementation. The standard library
// implements the ToString trait on any type that implements the Display
// trait. This means that the to_string method defined by the ToString trait
// can be called on any type that implments the Display trait

// let s = 3.to_string();

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
