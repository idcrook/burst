fn main() {
    let some_option_value: Option<i32> = Some(5);
    // let Some(x) = some_option_value; // not allowed
    if let Some(x) = some_option_value {
        // allowed
        println!("{}", x);
    }
}
