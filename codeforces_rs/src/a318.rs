use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let mut input = get_input().trim().to_string();

    let mut substr_iter = input.split_whitespace();

    let mut next_num = || -> i64 {
        substr_iter
            .next()
            .expect("Not enough input numbers")
            .parse()
            .expect("Input is not a number")
    };

    let n = next_num();
    let k = next_num();

    if n % 2 == 0 {
        if k <= n / 2 {
            println!("{}", k * 2 - 1);
        } else {
            println!("{}", (k - n / 2) * 2);
        }
    } else {
        if k <= (n / 2 + 1) {
            println!("{}", k * 2 - 1);
        } else {
            println!("{}", (k - n / 2 - 1) * 2);
        }
    }

    Ok(())
}
