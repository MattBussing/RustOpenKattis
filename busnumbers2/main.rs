// https://open.kattis.com/problems/busnumbers2
// psuedo code:
//get cubes up to the limit
// add every combo of cubes and put them in a list (number and count)
// find the largest pair and return that

use std::collections::BinaryHeap;
use std::collections::HashMap;

use std::io;

fn main() {
    let limit = get_int();
    let mut cubes: Vec<i32> = Vec::new();
    let limit_3_root: i32 = (limit as f64).powf(1.0 / 3.0).ceil() as i32;
    let mut counts: HashMap<i32, i32> = HashMap::new();
    let mut totals = BinaryHeap::new();

    for i in 0..limit_3_root {
        cubes.push((i as i32).pow(3));
    }

    let end = cubes.len();
    for i in 0..end {
        for j in (i + 1)..end {
            let temp = cubes[i] + cubes[j];
            if counts.contains_key(&temp) {
                if let Some(count) = counts.get_mut(&temp) {
                    *count += 1;
                }
            } else {
                counts.insert(temp, 1);
                totals.push(temp);
            }
        }
    }

    let vec = totals.into_sorted_vec();

    let mut found = false;
    if vec.len() > 0 {
        let mut i = vec.len();
        while i >= 1 {
            i -= 1;
            if vec[i] < limit && counts[&vec[i]] >= 2 {
                println!("{}", vec[i]);
                found = true;
                break;
            }
        }
    }
    if !found {
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
