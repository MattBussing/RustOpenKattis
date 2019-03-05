// https://open.kattis.com/problems/skocimis

// use std::cmp::max;
// use std::cmp::min;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let mut numbers = String::new();
    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");
    let mut iter = numbers.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();

    let heap = BinaryHeap::from(vec![a, b, c]);
    let mut vec = heap.into_sorted_vec();
    let mut bot = vec[0];
    let mut mid = vec[1];
    let mut top = vec[2];

    let mut count = 0;
    // both solves
    // if top - mid < mid - bot {
    //     // go descending
    //     loop {
    //         if bot == mid - 1 {
    //             break;
    //         }
    //         let temp = mid;
    //         top = mid;
    //         mid = temp - 1;
    //         count += 1;
    //     }
    // } else {
    //     // go ascending
    //     loop {
    //         if top == mid + 1 {
    //             break;
    //         }
    //         let temp = mid;
    //         bot = mid;
    //         mid = temp + 1;
    //         count += 1;
    //     }
    // }
    if top - mid < mid - bot {
        let temp1 = bot;
        mid = -1 * mid;
        bot = -1 * top;
        top = -1 * temp1;
    }
    loop {
        if top == mid + 1 {
            break;
        }
        let temp = mid;
        bot = mid;
        mid = temp + 1;
        count += 1;
    }

    println!("{}", count);
}
