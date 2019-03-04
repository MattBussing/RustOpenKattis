// https://open.kattis.com/problems/welcomehard

use std::io;

// psuedo code:
// ?? how does the last four digits matter over others?
// start at the beginning of the string iterating over each letter
// find the first letter and add it to the stack go on to find the next letter
// so on till completed
//  -> count+1
// pop off the stack
// iterate over continue until found next letter or at the end
// then pop off stack
// stop when no letters are on the stack and at the end of the string

fn main() {
    let mut _number_cases = String::new();
    io::stdin()
        .read_line(&mut _number_cases)
        .ok()
        .expect("read error");
    let mut iter = _number_cases.split_whitespace();
    let _number_cases: i32 = iter.next().unwrap().parse().unwrap();
    for i in 0.._number_cases {
        let mut string = String::new();
        io::stdin().read_line(&mut string).ok().expect("read error");
        find_occurrences(string);
    }
}

fn find_occurrences(string: String) {
    println!("{}", string);
}
