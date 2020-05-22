use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let mut n: i64 = get_input().trim().parse().unwrap();

    let mut input = get_input().trim().to_string();

    let mut substr_iter = input.split_whitespace();

    let mut next_num = || -> usize {
        substr_iter
            .next()
            .expect("Not enough input numbers")
            .parse()
            .expect("Input is not a number")
    };

    let mut res = 0;
    let mut current = 0;
    let mut last = 0;
    for i in 0..n {
        // v.push(next_num());
        let d = next_num();
        // dbg!(d, last, current);

        if d >= last {
            current += 1;
        } else {
            if current > res {
                res = current;
            }
            current = 1;
        }
        last = d;
    }

    if current > res {
        res = current;
    }

    println!("{}", res);

    Ok(())
}
