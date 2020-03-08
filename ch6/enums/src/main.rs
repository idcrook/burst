// #![allow(unused_variables)]
// fn main() {
//     enum Message {
//         Quit,                       // no associated data
//         Move { x: i32, y: i32 },    // anonymous struct in
//         Write(String),              // single String
//         ChangeColor(i32, i32, i32), // three i32 values
//     }
// }

// #![allow(unused_variables)]
// fn main() {
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct
// }

#![allow(unused_variables)]
fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
