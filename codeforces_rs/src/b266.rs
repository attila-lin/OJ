use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_next_t(s: &mut String) {
    let mut res = s.chars().collect();

    let res2 = vec![];

    for i in 1..res.len() {
        if res[i - 1] == 'B' && res[i] == 'G' {
            res2.push('G');
            res2.push('B');
        }
    }
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

    let n = next_num();
    let t = next_num();

    let mut res = get_input().trim().to_string();

    for i in 0..t {
        get_next_t(&mut res);
    }

    println!("{}", res);

    Ok(())
}
