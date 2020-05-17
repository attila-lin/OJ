use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let mut input: i64 = get_input().trim().parse().unwrap();
    let mut res = 0;

    res += input / 5;
    input %= 5;

    res += input / 4;
    input %= 4;

    res += input / 3;
    input %= 3;

    res += input / 2;
    input %= 2;

    res += input / 1;

    println!("{}", res);

    Ok(())
}
