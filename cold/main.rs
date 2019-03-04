// https://open.kattis.com/problems/cold

// use std::collections::BinaryHeap;
// use std::collections::HashMap;

use std::io;

fn main() {
    let limit = get_int();
    let mut temperatures = get_ints();
    let mut count = 0;
    for i in temperatures {
        if i < 0 {
            count += 1;
        }
    }
    println!("{}", count)
    // let mut cubes: Vec<i32> = Vec::new();
    // let limit_3_root: i32 = (limit as f64).powf(1.0 / 3.0).ceil() as i32;
    // let mut counts: HashMap<i32, i32> = HashMap::new();
    // let mut totals = BinaryHeap::new();
}

fn get_int() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let mut x: i32 = 0;
    match trimmed.parse::<i32>() {
        Ok(i) => x = i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    return x;
}

fn get_ints() -> Vec<i32> {
    // https://stackoverflow.com/questions/26536871/convert-a-string-of-numbers-to-an-array-vector-of-ints-in-rust
    let mut numbers = String::new();

    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");

    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    return numbers;
}
