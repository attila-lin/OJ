use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn check_is_luck(input: &str) -> bool {
    let mut is_luck = true;
    for c in input.chars() {
        let n: usize = c.to_string().parse().unwrap();
        if n == 4 || n == 7 {
        } else {
            is_luck = false;
            break;
        }
    }

    is_luck
}

fn main() -> io::Result<()> {
    let mut input = get_input().trim().to_string();

    let is_luck = check_is_luck(&input);

    if is_luck {
        println!("YES");
        return Ok(());
    }

    let mut count = 0;
    for c in input.chars() {
        let n: usize = c.to_string().parse().unwrap();
        if n == 4 || n == 7 {
            count += 1;
        }
    }

    if check_is_luck(&count.to_string()) {
        println!("YES");
    } else {
        println!("NO");
    }

    Ok(())
}
