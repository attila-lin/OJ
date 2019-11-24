struct Solution;



impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ret = Vec::new();
        let mut count = 0;
        let len = strs[0].len();
        loop {
            let c = strs[0].chars().nth(count).unwrap();
            let mut success = true;
            for s in &strs {
                if s.chars().nth(count).unwrap() != c {
                    success = false;
                    break;
                }
            }

            if success {
                // println!("ggg");
                ret.push(c);
            }else {
                break;
            }

            count += 1;

            if len <= count {
                //  println!("ggg?{} {}", count, len);
                break;
            }
        }

        return ret.iter().cloned().collect::<String>();
    }
}

fn main() {
    let mut res = Solution::longest_common_prefix(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()]);
    println!("{}", res);

}