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

fn get_expected_amounts(mut divider: u32, mut total: f32, expected_amounts: &mut Vec<f32>) -> f32 {
    while divider > 0 {
        let mut amount: f32 = total / divider as f32;
        // round to nearest two decimal places.
        amount = amount.round_to_nearest_decimal();

        expected_amounts.push(amount);
        total -= amount;
        divider -= 1;
    }
    expected_amounts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

    return total;
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
    use std::{
        env, fs, io,
        path::{Path, PathBuf},
        str,
    };
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
    fn test_6() {
        // Divide out the money.
        // This will be a vector representing how much we give each person.
        let mut expected_amounts: Vec<f32> = Vec::new();
        let divider = 9;
        let total: f32 = 10.;
        let new_total = get_expected_amounts(divider, total, &mut expected_amounts);

        let new_total2: f32 = expected_amounts.iter().sum();
        assert_eq!(10.0, new_total2);

        assert_eq!(true, new_total.abs() < 0.001, "{}", new_total.abs());
    }

    #[test]
    fn test_7() {
        // Divide out the money.
        // This will be a vector representing how much we give each person.
        let mut expected_amounts: Vec<f32> = Vec::new();
        let divider = 2;
        let total: f32 = 3.01;
        let new_total = get_expected_amounts(divider, total, &mut expected_amounts);

        let new_total2: f32 = expected_amounts.iter().sum();
        assert_eq!(3.01, new_total2);

        assert_eq!(true, new_total.abs() < 0.001, "{}", new_total.abs());
    }

    pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
        let path = path.as_ref();

        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            env::current_dir()?.join(path)
        };

        Ok(absolute_path)
    }
    #[test]
    fn test_8() {
        let mut output: Vec<u8> = Vec::new();

        let path_string = String::from("./trip/data.dat");
        let temp_path = absolute_path(Path::new(&path_string)).expect("absolute_path failed");
        let path: &str = temp_path.to_str().expect("to_str failed");

        let contents = fs::read_to_string(path)
            .expect(&format!("Should have been able to read the file {}", path));

        main3(&mut contents.as_bytes(), &mut output);
        assert_eq!(
            str::from_utf8(&output).unwrap(),
            "$1214050.92\n$1284504.22\n$1268887.29\n$1262307.92\n$1237205.52\n$1281688.38\n$1234121.61\n$1241904.46\n$1290084.73\n$1234205.89"
        );
    }

    // http://acm.student.cs.uwaterloo.ca/~acm00/990131.data/
}
