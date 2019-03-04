// https://open.kattis.com/problems/welcomehard

use std::io;

// ?? how does the last four digits matter over others?

// psuedo code:
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
    let mut cases = Vec::new();
    for _i in 0.._number_cases {
        let mut string = String::new();
        io::stdin().read_line(&mut string).ok().expect("read error");
        cases.push(string.trim().to_string().chars().collect());
    }
    for i in 1..(_number_cases as usize + 1) {
        let x = cases.remove(0);
        // let temp = find_occurrences(x);
        // println!("Case #{}: {:04}", i, temp);
        let temp = recursive_outer(x);
        println!("Case #{}: {:04}", i, temp);
    }
}

// fn find_occurrences(string: Vec<char>) -> i32 {
//     let goal: Vec<char> = "welcome to code jam".chars().collect();
//     let mut count: i32 = 0;
//     let mut stack = Vec::<usize>::new();
//     let mut goal_pointer = 0;
//     let mut string_pointer = 0;
//     loop {
//         let goal_letter = goal[goal_pointer];
//         let letter = string[string_pointer];
//
//         // if the letters are equal
//         if letter == goal_letter {
//             // if at the end
//             if goal_pointer + 1 == goal.len() {
//                 // println!("success!");
//                 count += 1;
//             // the end is taken care of in the following if
//             } else {
//                 // other wise increase the goal_pointer
//                 stack.push(string_pointer);
//                 goal_pointer += 1;
//             }
//         }
//         string_pointer += 1;
//
//         while string_pointer >= string.len() {
//             if stack.is_empty() {
//                 return count;
//             }
//             string_pointer = stack.pop().unwrap() + 1;
//             goal_pointer -= 1;
//         }
//     }
// }

fn recursive_outer(string: Vec<char>) -> i32 {
    let goal: Vec<char> = "welcome to code jam".chars().collect();
    let goal_pointer = 0;
    let string_pointer = 0;
    let g_len = goal.len() - 1;
    let mut dp_table: Vec<Vec<i32>> = Vec::new();
    for _i in 0..string.len() + 1 {
        let mut temp = Vec::new();
        for _j in 0..goal.len() + 1 {
            temp.push(-1)
        }
        dp_table.push(temp);
    }
    // let mut dp_table: [[i32; string.len()]; goal.len()] = [[-1; string.len()]; goal.len()];
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
    dp_table: &mut Vec<Vec<i32>>,
) -> i32 {
    if s_index >= string.len() {
        return 0;
    }
    let goal_letter = goal[g_index];
    let letter = string[s_index];
    let mut count = 0;
    // if the letters are equal
    if letter == goal_letter {
        // if at the end
        if g_index == g_len {
            return 1;
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
    // count += recursive(&string, &goal, g_index, s_index + 1, g_len, &mut dp_table);
    count += dp_table[s_index + 1][g_index];

    return count;
}
