use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_res(i: u64) -> u64 {
    match i {
        0 | 1 | 2 => 0,
        _ => i - i / 2 - 1,
    }
}

fn main() -> io::Result<()> {
    let mut input: u64 = get_input().trim().parse().unwrap();

    for i in 0..input {
        let i: u64 = get_input().trim().parse().unwrap();
        let res = get_res(i);
        println!("{}", res);
    }

    Ok(())
}
