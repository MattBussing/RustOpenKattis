// https://open.kattis.com/problems/tomjerry
// this is pascal's triangle

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::{Index, IndexMut};

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
        let i: i32 = iter.next().unwrap().parse().unwrap();
        let j: i32 = iter.next().unwrap().parse().unwrap();
        // // println!("{},{}", i, j);
        cheese.push((j - 1, i - 1));
    }
    let mut lookup_table = table::new();
    lookup_table[(0, 0)] = 1;
    println!(
        "{}",
        binom(cheese[0].0 as u32, cheese[0].1 as u32, lookup_table)
    );
}

// struct table<'a> {
//     map: &'a mut HashMap<(u32, u32), u32>,
//     set: &'a mut HashSet<(u32, u32)>,
// }
//
// impl table<'a> {
//     fn new() -> table<'a> {
//         table {
//             map: &'a HashMap::new(),
//             set: &'a HashSet::new(),
//         }
//     }
// }
struct table {
    map: HashMap<(u32, u32), u32>,
    set: HashSet<(u32, u32)>,
}

impl table {
    fn new() -> table {
        table {
            map: HashMap::new(),
            set: HashSet::new(),
        }
    }
}

trait Contains {
    fn contains(&self, tuple: (u32, u32)) -> bool;
    fn get(&self, tuple: (u32, u32)) -> u32;
}
impl Contains for table {
    fn contains(&self, tuple: (u32, u32)) -> bool {
        return self.set.contains(&tuple);
    }
    fn get(&self, tuple: (u32, u32)) -> u32 {
        return self.map[&tuple];
    }
}
impl Index<(u32, u32)> for table {
    type Output = u32;
    fn index(& self, tuple: (u32, u32)) -> &Self::Output {
        self.set.insert(tuple);
        return &self.map[&tuple];
    }
}
impl IndexMut<(u32, u32)> for table {
    // type Output = u32;
    fn index_mut<'a>(&'a mut self, tuple: (u32, u32)) -> &'a mut u32 {
        self.set.insert(tuple);
        return &mut self.map[&tuple];
    }
}
// impl table {
//     fn new(&self) {
//         self.map = HashMap::new();
//         self.set = HashSet::new();
//     }
// }

fn binom(n: u32, k: u32, set:HashSet, map:HashMap) -> u32 {
    // n choose k
    if !lookup_table.contains((n, k)) {
        if k == 0 || k == n {
            return 1;
        }
        lookup_table[(n, k)] = binom(n - 1, k - 1, lookup_table) + binom(n - 1, k, lookup_table);
    }
    return lookup_table[(n, k)];
}
