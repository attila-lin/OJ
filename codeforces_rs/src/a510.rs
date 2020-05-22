use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let mut input = get_input().trim().to_string();

    let mut substr_iter = input.split_whitespace();

    let mut next_num = || -> usize {
        substr_iter
            .next()
            .expect("Not enough input numbers")
            .parse()
            .expect("Input is not a number")
    };

    let a = next_num();
    let b = next_num();

    for i in 0..a {
        for j in 0..b {
            if i % 2 != 1 {
                print!("#");
            } else {
                if j == b - 1 && (i / 2) % 2 == 0 {
                    print!("#");
                } else if j == 0 && (i / 2) % 2 == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        print!("\n");
    }

    Ok(())
}
