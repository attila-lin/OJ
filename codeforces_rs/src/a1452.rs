use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let count: i16 = get_input().trim().parse().unwrap();

    for i in 0..count {
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

        let min = std::cmp::min(n, k);
        let max = std::cmp::max(n, k);

        let sub = max - min;
        if sub > 0 {
            println!("{}", sub * 2 - 1 + min * 2);
        }
        else {
            println!("{}", min * 2);
        }
        
    }

    Ok(())
}
