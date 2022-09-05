// https://open.kattis.com/problems/pot

use std::io;

fn main() {
    let mut numbers = String::new();
    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");
    let mut iter = numbers.split_whitespace();
    let x: i32 = iter.next().unwrap().parse().unwrap();
    let y: i32 = iter.next().unwrap().parse().unwrap();
    for i in 1..(x + 1) {
        let mut _number_cases = String::new();
        io::stdin()
            .read_line(&mut _number_cases)
            .ok()
            .expect("read error");
    }
    println!("{}", y);
}
