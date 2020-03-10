fn main() {
    //panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99];
}
// emacs:
// prefix argument on cargo-process-run to edit `cargo run` command
//     C-u C-c C-c C-r
// then prepend with:
//     RUST_BACKTRACE=1 ...
