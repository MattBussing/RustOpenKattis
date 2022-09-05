// https://open.kattis.com/problems/trip

use std::{
    cmp::Ordering::Equal,
    fmt::Debug,
    io::{self, BufRead, StdinLock, Write},
    str::FromStr,
};

fn main() {
    let stdin1 = io::stdin();
    let mut stdin: StdinLock = stdin1.lock();
    main3(&mut stdin, &mut io::stdout());
}

fn main3(my_stdin: &mut impl BufRead, output: &mut impl Write) {
    let mut buffer: String = "".to_owned();

    loop {
        if !buffer.is_empty() {
            buffer = format!("{}\n", buffer)
        }

        let num_people: u32 = get_single_item(my_stdin);
        if num_people == 0 {
            break;
        }

        // Get inputs into an array.
        let mut vec: Vec<f32> = Vec::new();
        for _ in 0..(num_people) {
            vec.push(get_single_item(my_stdin));
        }

        // sort the array
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

        // let mut vec: Vec<f32> = Vec::new();
        // for i in vec.iter() {
        //     average += i;
        // }
        // average = average / num_people as f32;

        // Divide out the money.
        // This will be a vector representing how much we give each person.
        let mut division: Vec<f32> = Vec::new();
        let mut divider = num_people;
        let mut total: f32 = vec.iter().sum();
        while divider > 0 {
            let mut amount: f32 = total / divider as f32;
            amount = amount.round();

            division.push(amount);
            total -= amount;
            divider -= 1;
        }

        // Get the ammount needed to transfer.
        // I think we can only look at the bottom half.
        let mut total_money_transferred: f32 = 0.0;
        for (pos, e) in vec.iter().enumerate() {
            let desired: f32 = division[pos];
            if e < &desired {
                total_money_transferred += desired - e;
            }
        }

        buffer = format!("{}${:.2}", buffer, total_money_transferred)
    }

    output.write_all(buffer.as_bytes()).ok();
}

fn get_single_item<T>(my_stdin: &mut impl BufRead) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    my_stdin.read_line(&mut input).expect("Failed to read line");
    let x: T = input.trim().parse().unwrap();
    return x;
}

// #[cfg(test)]
// mod tests {

//     use main3;
//     use std::str;

//     #[test]
//     fn test_1() {
//         let mut output: Vec<u8> = Vec::new();

//         main3(
//             &mut r"3
// 10.00
// 20.00
// 30.00
// 4
// 15.00
// 15.01
// 3.00
// 3.01
// 0"
//             .as_bytes(),
//             &mut output,
//         );
//         assert_eq!(str::from_utf8(&output).unwrap(), "$10.00\n$11.99\n");
//     }
// }
