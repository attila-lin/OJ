use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let mut input = get_input().trim().to_string();

    let mut input2 = get_input().trim().to_string();

    let res: String = input.chars().rev().collect();

    if input2 == res {
        println!("YES");
    } else {
        println!("NO");
    }

    Ok(())
}
