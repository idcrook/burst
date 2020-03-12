#![allow(unused_variables)]
use std::thread;
use std::time::Duration;

// implement memoization / lazy evaluation
struct Cacher<T>
where
    T: Fn(u32) -> u32, // Fn trait
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 35;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    //assert_eq!(v2, 2);
    assert_eq!(v2, 1);
}

// IDEAS:

// Try modifying Cacher to hold a hash map rather than a single value. The keys of the hash map will be the arg values that are passed in, and the values of the hash map will be the result of calling the closure on that key. Instead of looking at whether self.value directly has a Some or a None value, the value function will look up the arg in the hash map and return the value if it’s present. If it’s not present, the Cacher will call the closure and save the resulting value in the hash map associated with its arg value.

// The second problem with the current Cacher implementation is that it only accepts closures that take one parameter of type u32 and return a u32. We might want to cache the results of closures that take a string slice and return usize values, for example. To fix this issue, try introducing more generic parameters to increase the flexibility of the Cacher functionality.
