use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let input1 = get_input().trim().to_string();
    let input2 = get_input().trim().to_string();

    for i in 0..input1.len() {
        let c1 = input1.chars().nth(i);
        let c2 = input2.chars().nth(i);
        if c1 == c2 {
            print!("{}", 0);
        } else {
            print!("{}", 1);
        }
    }
    print!("\n");

    Ok(())
}
