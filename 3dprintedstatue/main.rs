use std::io;

fn main() {
    // https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0
    let mut input_text = String::new();
    // println!("please input a number");
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let mut x: i32 = 0;
    match trimmed.parse::<i32>() {
        Ok(i) => x = i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    // println!("this was an integer: {}", x);
    let y: f64 = x.into();
    println!("{}", y.log(2.0).ceil() + 1.0);
}
