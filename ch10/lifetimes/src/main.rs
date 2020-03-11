#![allow(unused_variables)]
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Lifetime annotations have a slightly unusual syntax: the names of lifetime
// parameters must start with an apostrophe (') and are usually all lowercase
// and very short, like generic types.

// Lifetime Annotations in Function Signatures

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let r;
    {
        let x = 5;
        // error[E0597]: `x` does not live long enough
        r = &x;
        //  ^^ borrowed value does not live long enough
        println!("r: {}", r);
    }

    //println!("r: {}", r);
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result); // illegal borrow (string2) used here

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{}", i.part);
}
