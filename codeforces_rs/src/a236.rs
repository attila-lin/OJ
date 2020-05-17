use std::collections::HashSet;
use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let input: String = get_input().trim().to_string();

    let mut set = HashSet::new();
    for c in input.chars() {
        set.insert(c);
    }

    if set.len() % 2 == 0 {
        println!("CHAT WITH HER!");
    } else {
        println!("IGNORE HIM!");
    }

    Ok(())
}
