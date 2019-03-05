// https://open.kattis.com/problems/tomjerry
// dynamic programming challenge

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
    let mut cheese = HashSet::new();
    for _i in 0.._number_cases {
        let mut string = String::new();
        io::stdin().read_line(&mut string).ok().expect("read error");
        let mut iter = string.split_whitespace();
        let i: i32 = iter.next().unwrap().parse().unwrap();
        let j: i32 = iter.next().unwrap().parse().unwrap();
        // // println!("{},{}", i, j);
        cheese.insert((j - 1, i - 1));
    }

    // println!("cheese: {:?}", cheese);

    // test
    // // println!("{},{},{}", h, w, _number_cases);

    // create dp table
    let mut count: Vec<Vec<(i32, i32)>> = Vec::new();
    for _i in 0..h {
        let mut temp = Vec::new();
        for _j in 0..w {
            temp.push((0, 0));
        }
        count.push(temp);
    }
    // problem is c, r but we will index r, c
    // (cheese, non-cheese)
    let mut i = h - 1;
    while i >= 0 {
        let mut j = w - 1;
        while j >= 0 {
            // println!("{},{}", i, j);
            if i == h - 1 && j == w - 1 {
                // println!("&&");
                if cheese.contains(&(i, j)) {
                    count[i as usize][j as usize] = (1, 0);
                } else {
                    count[i as usize][j as usize] = (0, 1);
                }
            } else if i == h - 1 {
                // println!("h");

                if cheese.contains(&(i, j)) {
                    // println!("chees");
                    // add the cell to the right and the one below
                    // move all into cheese cell
                    count[i as usize][j as usize] = (
                        count[i as usize][(j + 1) as usize].0
                            + count[i as usize][(j + 1) as usize].1,
                        0,
                    );
                } else {
                    count[i as usize][j as usize] = count[i as usize][(j + 1) as usize]
                }
            } else if j == w - 1 {
                // println!("w");
                if cheese.contains(&(i, j)) {
                    // println!("chees");
                    // add the cell to the right and the one below
                    // move all into cheese cell
                    count[i as usize][j as usize] = (
                        count[(i + 1) as usize][j as usize].0
                            + count[(i + 1) as usize][j as usize].1,
                        0,
                    );
                } else {
                    count[i as usize][j as usize] = count[(i + 1) as usize][j as usize]
                }
            } else {
                if cheese.contains(&(i, j)) {
                    // add the cell to the right and the one below
                    // move all into cheese cell
                    count[i as usize][j as usize] = (
                        count[i as usize][(j + 1) as usize].0
                            + count[i as usize][(j + 1) as usize].1
                            + count[(i + 1) as usize][j as usize].0
                            + count[(i + 1) as usize][j as usize].1,
                        0,
                    );
                } else {
                    count[i as usize][j as usize] = (
                        count[i as usize][(j + 1) as usize].0
                            + count[(i + 1) as usize][j as usize].0,
                        count[i as usize][(j + 1) as usize].1
                            + count[(i + 1) as usize][j as usize].1,
                    );
                }
            }
            j -= 1;
        }
        i -= 1;
    }
    // // println!()

    // for i in 0..h as usize {
    //     for j in 0..w as usize {
    //         print!("{},{}|", count[i][j].0, count[i][j].1);
    //     }
    //     print!("\n\n");
    // }
    println!("{}", count[0][0].0);
}

// fn
