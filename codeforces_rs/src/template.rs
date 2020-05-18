use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let mut input: i64 = get_input().trim().parse().unwrap();

    let mut substr_iter = input.split_whitespace();

    let mut next_num = || -> usize {
        substr_iter
            .next()
            .expect("Not enough input numbers")
            .parse()
            .expect("Input is not a number")
    };

    Ok(())
}
