use prisms::highlight_str;

#[test]
fn highlight_str_test() {
    let a = highlight_str!("let two = 1 + 1;", "javascript");
    println!("{}", a);
}
