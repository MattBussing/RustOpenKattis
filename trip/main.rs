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

        // Divide out the money.
        // This will be a vector representing how much we give each person.
        let mut expected_amounts: Vec<f32> = Vec::new();
        let divider = num_people;
        let total: f32 = vec.iter().sum();
        get_expected_amounts(divider, total, &mut expected_amounts);

        // Get the ammount needed to transfer.
        // I think we can only look at the bottom half.
        let mut total_money_transferred: f32 = 0.0;
        for (pos, e) in vec.iter().enumerate() {
            let desired: f32 = expected_amounts[pos];
            if e < &desired {
                total_money_transferred += desired - e;
            }
        }

        buffer = format!("{}${:.2}", buffer, total_money_transferred)
    }

    output.write_all(buffer.as_bytes()).ok();
}

fn get_expected_amounts(mut divider: u32, mut total: f32, expected_amounts: &mut Vec<f32>) {
    while divider > 0 {
        let mut amount: f32 = total / divider as f32;
        // round to nearest two decimal places.
        amount = amount.round_to_nearest_decimal();

        expected_amounts.push(amount);
        total -= amount;
        divider -= 1;
    }
    expected_amounts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
}

pub trait Round {
    fn round_to_nearest_decimal(&self) -> f32;
}

impl Round for f32 {
    fn round_to_nearest_decimal(&self) -> f32 {
        return (self * 100.0).round() / 100.0;
    }
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

#[cfg(test)]
mod tests {

    use get_expected_amounts;
    use main3;
    use std::str;
    use Round;

    #[test]
    fn test_1() {
        let mut output: Vec<u8> = Vec::new();

        main3(
            &mut r"3
10.00
20.00
30.00
4
15.00
15.01
3.00
3.01
0"
            .as_bytes(),
            &mut output,
        );
        assert_eq!(str::from_utf8(&output).unwrap(), "$10.00\n$11.99\n");
    }

    #[test]
    fn test_2() {
        let mut output: Vec<u8> = Vec::new();

        main3(
            &mut r"4
3.01
3.01
3.00
3.01
0"
            .as_bytes(),
            &mut output,
        );
        assert_eq!(str::from_utf8(&output).unwrap(), "$0.00\n");
    }

    #[test]
    fn test_3() {
        let mut output: Vec<u8> = Vec::new();

        main3(
            &mut r"8
3.01
3.01
3.01
3.01
3.02
3.01
3.00
3.00
0"
            .as_bytes(),
            &mut output,
        );
        assert_eq!(str::from_utf8(&output).unwrap(), "$0.01\n");
    }

    #[test]
    fn test_4() {
        let mut output: Vec<f32> = Vec::new();
        // let mut expected_amounts: Vec<f32> = Vec::new();

        get_expected_amounts(8, 31.0, &mut output);
        assert_eq!(
            output[0].round_to_nearest_decimal(),
            3.87.round_to_nearest_decimal()
        );
        assert_eq!(
            output[4].round_to_nearest_decimal(),
            3.88.round_to_nearest_decimal()
        );
    }

    #[test]
    fn test_5() {
        let mut output: Vec<u8> = Vec::new();

        main3(
            &mut r"3
10.00
20.00
30.00
4
15.00
15.01
3.00
3.01
6
15.00
15.00
9.00
9.00
3.02
3.01
0"
            .as_bytes(),
            &mut output,
        );
        assert_eq!(str::from_utf8(&output).unwrap(), "$10.00\n$11.99\n$11.98\n");
    }

    #[test]
    fn test_5() {
        let mut output: Vec<u8> = Vec::new();

        main3(
            // http://acm.student.cs.uwaterloo.ca/~acm00/990131.data/A.2.dat
            &mut r"3
10.00
20.00
30.00
4
15.00
15.01
3.00
3.01
6
15.00
15.00
9.00
9.00
3.02
3.01
0"
            .as_bytes(),
            &mut output,
        );
        assert_eq!(str::from_utf8(&output).unwrap(), "$10.00\n$11.99\n$11.98\n");
    }

    //http://acm.student.cs.uwaterloo.ca/~acm00/990131.data/A.3.dat
    #[test]
    fn test_5() {
        let mut output: Vec<u8> = Vec::new();

        main3(
            &mut r"3
10.00
20.00
30.00
4
15.00
15.01
3.00
3.01
6
15.00
15.00
9.00
9.00
3.02
3.01
0"
            .as_bytes(),
            &mut output,
        );
        assert_eq!(str::from_utf8(&output).unwrap(), "$10.00\n$11.99\n$11.98\n");
    }

    // todo make sure total has 0
    #[test]
    fn test_5() {
        let mut output: Vec<u8> = Vec::new();

        main3(
            &mut r"3
10.00
20.00
30.00
4
15.00
15.01
3.00
3.01
6
15.00
15.00
9.00
9.00
3.02
3.01
0"
            .as_bytes(),
            &mut output,
        );
        assert_eq!(str::from_utf8(&output).unwrap(), "$10.00\n$11.99\n$11.98\n");
    }
    // http://acm.student.cs.uwaterloo.ca/~acm00/990131.data/
}
