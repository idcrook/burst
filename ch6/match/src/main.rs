#![allow(unused_variables)]
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    // // pattern `None` not covered
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // }

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        8u8..=std::u8::MAX => println!("others"), //_ => (),
        _ => (),
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
