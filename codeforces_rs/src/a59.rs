use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let mut input = get_input().trim().to_string();

    let mut count = 0;
    for c in input.chars() {
        if c.is_ascii_uppercase() {
            count += 1;
        }
    }

    if count > input.len() / 2 {
        println!("{}", input.to_uppercase());
    } else {
        println!("{}", input.to_lowercase());
    }

    Ok(())
}
