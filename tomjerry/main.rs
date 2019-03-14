// https://open.kattis.com/problems/tomjerry
// this is pascal's triangle

use std::collections::HashMap;
use std::collections::HashSet;

use std::io;

fn main() {
    // get h w
    let mut numbers = String::new();
    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");
    let mut iter = numbers.split_whitespace();
    let w: i32 = iter.next().unwrap().parse().unwrap();
    let h: i32 = iter.next().unwrap().parse().unwrap();

    // get n
    let mut _number_cases = String::new();
    io::stdin()
        .read_line(&mut _number_cases)
        .ok()
        .expect("read error");
    let mut iter = _number_cases.split_whitespace();
    let _number_cases: i32 = iter.next().unwrap().parse().unwrap();

    // get all the cheese
    let mut cheese = Vec::new();
    for _i in 0.._number_cases {
        let mut string = String::new();
        io::stdin().read_line(&mut string).ok().expect("read error");
        let mut iter = string.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        cheese.push((x, y));
    }

    cheese.sort();
    for i in 0..cheese.len() {
        println!(
            "for loop: binoms{} {}",
            cheese[i].0,
            cheese[i].1 // binom((i.0).0 as u32, (i.0).1 as u32, &mut set, &mut map),
                        // binom((i.1).0 as u32, (i.1).1 as u32, &mut set, &mut map)
        );
        let mut j: i32 = i as i32 - 1;
        let subtrahend = 0;
        while j >= 0 {
            println!(
                "binoms{} {}",
                cheese[j as usize].0,
                cheese[j as usize].1 // binom((i.0).0 as u32, (i.0).1 as u32, &mut set, &mut map),
                                     // binom((i.1).0 as u32, (i.1).1 as u32, &mut set, &mut map)
            );
            j -= 1;
        }
    }

    // println!(
    //     "{}",
    //     binom(cheese[0].0 as u32, cheese[0].1 as u32, &mut set, &mut map)
    // );
}

fn binom(n: u32, k: u32, set: &mut HashSet<(u32, u32)>, map: &mut HashMap<(u32, u32), u32>) -> u32 {
    // n choose k
    // nth row and kth column
    // println!("{},{}", n, k);
    let mut x = 0;
    if !set.contains(&(n, k)) {
        if k == 0 || k == n {
            return 1;
        }
        x = binom(n - 1, k - 1, set, map) + binom(n - 1, k, set, map);
        // map[&(n, k)] = x;
        map.insert((n, k), x);
        set.insert((n, k));
    }
    return x;
}
