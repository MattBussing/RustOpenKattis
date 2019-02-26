use std::io;

fn main() {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed2 = input_text.trim();

    if trimmed.chars().count() < trimmed2.chars().count() {
        println!("no");
    } else {
        println!("go");
    }
}
