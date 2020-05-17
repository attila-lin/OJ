use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() -> io::Result<()> {
    let mut input = get_input().trim().to_string();

    let mut res = vec![];
    let mut first = true;
    for c in input.chars() {
        if first {
            res.push(c.to_ascii_uppercase());
            first = false;
        } else {
            res.push(c);
        }
    }

    let res: String = res.into_iter().collect();

    println!("{}", res);

    Ok(())
}
