// https://open.kattis.com/problems/welcomehard

use std::io;

fn main() {
    let mut _number_cases = String::new();
    io::stdin()
        .read_line(&mut _number_cases)
        .ok()
        .expect("read error");
    let mut iter = _number_cases.split_whitespace();
    let _number_cases: i128 = iter.next().unwrap().parse().unwrap();
    let mut cases = Vec::new();
    for _i in 0.._number_cases {
        let mut string = String::new();
        io::stdin().read_line(&mut string).ok().expect("read error");
        cases.push(string.trim().to_string().chars().collect());
    }
    for i in 1..(_number_cases as usize + 1) {
        let temp = recursive_outer(cases.remove(0));
        println!("Case #{}: {:04}", i, temp % 10000);
    }
}
fn recursive_outer(string: Vec<char>) -> i128 {
    let goal: Vec<char> = "welcome to code jam".chars().collect();
    let goal_pointer = 0;
    let string_pointer = 0;
    let g_len = goal.len() - 1;
    let mut dp_table: Vec<Vec<i128>> = Vec::new();
    for _i in 0..string.len() + 1 {
        let mut temp = Vec::new();
        for _j in 0..goal.len() + 1 {
            temp.push(-1)
        }
        dp_table.push(temp);
    }
    return recursive(
        &string,
        &goal,
        goal_pointer,
        string_pointer,
        g_len,
        &mut dp_table,
    );
}

fn recursive(
    string: &Vec<char>,
    goal: &Vec<char>,
    g_index: usize,
    s_index: usize,
    g_len: usize,
    dp_table: &mut Vec<Vec<i128>>,
) -> i128 {
    if s_index >= string.len() {
        return 0;
    }
    let mut count = 0;
    // if the letters are equal
    if string[s_index] == goal[g_index] {
        // if at the end
        if g_index == g_len {
            count = 1;
        } else {
            if dp_table[s_index + 1][g_index + 1] == -1 {
                dp_table[s_index + 1][g_index + 1] =
                    recursive(&string, &goal, g_index + 1, s_index + 1, g_len, dp_table);
            }
            count += dp_table[s_index + 1][g_index + 1];
        }
    }
    if dp_table[s_index + 1][g_index] == -1 {
        dp_table[s_index + 1][g_index] =
            recursive(&string, &goal, g_index, s_index + 1, g_len, dp_table);
    }
    count += dp_table[s_index + 1][g_index];

    return count;
}
