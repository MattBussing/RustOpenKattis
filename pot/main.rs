// https://open.kattis.com/problems/pot

use std::io;

fn main() {
    let mut first_input = String::new();
    io::stdin()
        .read_line(&mut first_input)
        .ok()
        .expect("read error");
    println!("first_input={}", first_input);
    let x: i32 = first_input.parse().unwrap();
    println!("x={}", x);

    // for i in 1..(x + 1) {
    //     let mut _number_cases = String::new();
    //     io::stdin()
    //         .read_line(&mut _number_cases)
    //         .ok()
    //         .expect("read error");
    // }
    // println!("{}", y);
}
