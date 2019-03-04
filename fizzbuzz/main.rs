// https://open.kattis.com/problems/fizzbuzz

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
    let n: i32 = iter.next().unwrap().parse().unwrap();
    // println!("{},{},{}", x, y, n);
    for i in 1..(n + 1) {
        let p1 = i % x == 0;
        let p2 = i % y == 0;
        if p1 && p2 {
            println!("FizzBuzz");
        } else if p1 {
            println!("Fizz");
        } else if p2 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}
