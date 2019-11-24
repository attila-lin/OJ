struct Solution;



impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);

        let c = s.chars();
        let mut ret = 0;
        let mut last = 0;
        for k in c {
            ret += map[&k];

            if last < map[&k] {
                ret -= 2 * last;
            }

            last = map[&k];
        }


        return ret;
    }
}

fn main() {
    let mut res = Solution::roman_to_int("IV".to_string());
    println!("{}", res);

    res = Solution::roman_to_int("III".to_string());
    println!("{}", res);

     res = Solution::roman_to_int("IX".to_string());
    println!("{}", res);

     res = Solution::roman_to_int("LVIII".to_string());
    println!("{}", res);


res = Solution::roman_to_int("MCMXCIV".to_string());
    println!("{}", res);
}