fn main() {
    //another_function(5);
    //another_function(5, 6);
    //let y = 6;
    // // let used as an expression - not valid in stable
    // let x = (let y = 6);

    let x = five();

    println!("The value of x is: {}", x);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x);
// }

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    //x + 1
    //x + 1;
    return x + 1;
}
