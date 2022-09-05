// https://open.kattis.com/problems/trip

use std::io::{self, BufRead, StdinLock, Write};

fn main() {
    let mut stdin: StdinLock = io::stdin().lock();
    main3(&mut stdin, &mut io::stdout());
}

fn main3(input: &mut impl BufRead, output: &mut impl Write) {
    let mut buffer = "".to_string();

    input.read_line(&mut buffer).ok();
    output.write_all(buffer.to_uppercase().as_bytes()).ok();
}

#[cfg(test)]
mod tests {

    use main3;
    use std::str;

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
}
