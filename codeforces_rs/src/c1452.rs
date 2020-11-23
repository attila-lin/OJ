use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let mut input: i64 = get_input().trim().parse().unwrap();

    for i in 0..input {
        let mut s = get_input().trim().to_string();

        let mut res = 0;

        let mut v1 = vec![];
        let mut v2 = vec![];
        for c in s.chars() {
            if c == '(' {
                v1.push(c);
            }
            if v1.len() > 0 && c == ')' && *v1.last().unwrap() == '(' {
                v1.pop();
                res += 1;
            }

            if c == '[' {
                v2.push(c);
            }
            if v2.len() > 0 && c == ']' && *v2.last().unwrap() == '[' {
                v2.pop();
                res += 1;
            }
        }

        println!("{}", res);
    }

    Ok(())
}
