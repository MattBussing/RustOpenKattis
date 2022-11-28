// https://open.kattis.com/problems/trip

use std::{
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
        let mut vec: Vec<u32> = Vec::new();
        for _ in 0..(num_people) {
            let item: f32 = get_single_item(my_stdin);
            vec.push((item * 100.0).round() as u32);
        }

        // Divide out the money.
        // This will be a vector representing how much we give each person.
        let sum: u32 = vec.iter().sum();
        let avg: u32 = sum / num_people;
        let remainder: u32 = sum % num_people;

        // Get the ammount needed to transfer.
        // I think we can only look at the bottom half.
        let mut total_money_transferred: u32 = 0;
        let mut number_over: u32 = 0;
        for (_, e) in vec.iter().enumerate() {
            if e > &avg {
                total_money_transferred += e - &avg;
                number_over += 1;
            }
        }

        let final_total: u32;
        if number_over >= remainder {
            // Can distribute all the remainder among those that moved money.
            final_total = total_money_transferred - remainder
        } else {
            // Can distribute the remainder among some of the people that moved money.
            final_total = total_money_transferred - number_over
        }

        // debug
        // let new_buf: String = format!(
        //     "{} {} num_people={} sum={} avg={} sum%n={} number_over={} if={}\n",
        //     total_money_transferred,
        //     final_total,
        //     num_people,
        //     sum,
        //     avg,
        //     remainder,
        //     number_over,
        //     number_over >= remainder
        // );
        // output.write_all(new_buf.as_bytes()).ok();

        buffer = format!("{}${}.{:02}", buffer, final_total / 100, final_total % 100)
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

#[cfg(test)]
mod tests {

    use main3;
    use std::{
        env, fs, io,
        path::{Path, PathBuf},
        str,
    };

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
            "$1214050.92\n$1284504.22\n$1268887.29\n$1262307.92\n$1237205.52\n$1281688.38\n$1234121.61\n$1241904.46\n$1290084.73\n$1234205.89\n"
        );
    }

    #[test]
    fn test_9() {
        let mut output: Vec<u8> = Vec::new();

        main3(
            &mut r"8
3.01
3.01
3.01
3.01
3.01
3.01
3.00
3.00
0"
            .as_bytes(),
            &mut output,
        );
        assert_eq!(str::from_utf8(&output).unwrap(), "$0.00\n");
    }

    #[test]
    fn test_10() {
        let mut output: Vec<u8> = Vec::new();

        main3(
            &mut r"8
3.01
3.01
3.01
3.01
3.01
3.01
3.01
2.99
0"
            .as_bytes(),
            &mut output,
        );
        assert_eq!(str::from_utf8(&output).unwrap(), "$0.01\n");
    }

    // http://acm.student.cs.uwaterloo.ca/~acm00/990131.data/
}
