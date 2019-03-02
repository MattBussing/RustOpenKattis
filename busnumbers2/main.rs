// https://open.kattis.com/problems/busnumbers2

use std::io;

// warning this is wrong!!!

fn main() {
    let mut found_limit = 0;
    let upper_limit = get_int();
    let mut cubes: Vec<i32> = Vec::new();
    let cube_root: i32 = (upper_limit as f64).powf(1.0 / 3.0).ceil() as i32;
    // let mut pair_1 = (-1, -1);
    // let mut pair_2 = (-1, -1);

    let mut flag = false;
    let mut double_flag = false;
    let mut pairs: Vec<(i32)> = Vec::new();

    for i in 0..cube_root {
        let possible = (i as i32).pow(3);
        // println!("i{} {}", i, possible);
        cubes.push(possible);
        let mut j = cubes.len() - 1;
        loop {
            // println!("temp, {},{}", temp, upper_limit);
            let temp = cubes[j] + possible;
            if temp < upper_limit {
                if temp == found_limit {
                    if flag == true {
                        // pairs.push((cubes[j]))
                        // println!("aaah");
                        // double_flag = true;
                    }
                    flag = true;
                } else if temp < found_limit {
                    break;
                } else {
                    found_limit = temp;
                    // pair_1 = (possible, cubes[j]);
                    flag = false;
                    double_flag = false;
                }
            }
            if j <= 0 {
                break;
            }
            j -= 1;
        }
    }
    if !double_flag && flag {
        println!("{}", pair_1.0 + pair_1.1);
    } else {
        println!("none");
    }
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
