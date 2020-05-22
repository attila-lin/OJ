use std::collections::HashMap;
use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let mut n: i64 = get_input().trim().parse().unwrap();

    let mut m = HashMap::new();
    for i in 0..n {
        let mut input = get_input().trim().to_string();
        let key = input.chars().nth(0);
        if m.contains_key(&key) {}

        if let Some(n) = m.get_mut(&key) {
            *n += 1;
        } else {
            m.insert(key, 1);
        }
    }

    let mut res = 0;
    for (k, v) in m {
        let min = if v == 1 {
            0
        } else {
            let mut min = i64::MAX;
            for i in 0..=v / 2 {
                let a = i * (i - 1) / 2;

                let left = v - i;
                let b = left * (left - 1) / 2;
                // dbg!(a + b);
                if min > a + b {
                    min = a + b;
                }
            }

            // dbg!(min);
            min
        };

        res += min;
    }

    println!("{}", res);

    Ok(())
}
