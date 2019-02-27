// https://open.kattis.com/problems/aboveaverage

use std::io;

fn main() {
    // get all the scores
    let size = get_int();
    let mut vec: Vec<Vec<i32>> = Vec::new();
    let mut sizes: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < size {
        let temp = get_ints();
        sizes.push(temp[0]);
        vec.push(temp[1..].to_vec());
        i += 1;
    }

    // find average score
    // println!("finding average sscore");
    let mut averages: Vec<f64> = Vec::new();
    for i in 0..vec.len() {
        let mut total = 0;
        for j in vec[i].clone() {
            total += j;
        }
        let ave: f64 = (total as f64) / (sizes[i] as f64);
        // println!("{}", ave);
        averages.push(ave);
    }

    // find number below average
    // println!("finding averages");
    for i in 0..vec.len() {
        let mut total: f64 = 0.0;
        for j in &vec[i] {
            if (*j as f64) > averages[i] {
                total += 1.0;
            }
        }

        println!("{:.3}%", total / sizes[i] as f64 * 100.0);
        // count.push(total);
    }

    // let y: f64 = x.into();
    // println!("{}", y.log(2.0).ceil() + 1.0);
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
