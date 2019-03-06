// https://open.kattis.com/problems/qaly

// use std::collections::HashSet;
use std::io;

fn main() {
    // get n
    let mut _number_cases = String::new();
    io::stdin()
        .read_line(&mut _number_cases)
        .ok()
        .expect("read error");
    let mut iter = _number_cases.split_whitespace();
    let _number_cases: i32 = iter.next().unwrap().parse().unwrap();

    let mut running_total: f64 = 0.0;
    // get all the cheese
    for _i in 0.._number_cases {
        let mut string = String::new();
        io::stdin().read_line(&mut string).ok().expect("read error");
        let mut iter = string.split_whitespace();
        let i: f64 = iter.next().unwrap().parse().unwrap();
        let j: f64 = iter.next().unwrap().parse().unwrap();
        running_total += i * j;
    }
    println!("{}", running_total);
}
