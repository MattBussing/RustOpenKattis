// https://open.kattis.com/problems/pot

use std::io;

fn main() {
    // print!("please type the number of items you plan on giving: ");

    let x: u32 = get_single_item_stdin();
    // println!("x={}", x);

    let mut total = 0;
    for _ in 0..(x) {
        let i: u32 = get_single_item_stdin();
        // 212 becomes 21^2
        let power: u32 = i % 10;
        let base: u32 = i / 10;

        total += base.pow(power);
    }

    println!("{}", total);
}

fn get_single_item_stdin() -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // println!("input={}", input);
    let x: u32 = input.trim().parse().expect("Input not an integer");
    // println!("x={}", x);

    return x;
}
