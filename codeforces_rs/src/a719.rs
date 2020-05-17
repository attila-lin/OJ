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

    let mut val1 = next_num();
    let mut val2 = next_num();

    // dbg!(val1, val2);
    let mut res = 1;
    loop {
        val1 *= 3;
        val2 *= 2;

        if val1 > val2 {
            break;
        }

        res += 1;
    }

    println!("{}", res);

    Ok(())
}
