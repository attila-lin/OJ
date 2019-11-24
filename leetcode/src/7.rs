struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // if x > std::i32::MAX || x < std::i32::MIN {
        //     return 0;
        // }

        let mut res = 0;
        let mut xtmp = x;

        let mut signure = 1;
        if x < 0 {
            signure = -1;
            xtmp = -1 * xtmp;
        }

        while xtmp != 0 {
            let mut tmp = xtmp % 10;
            res = res * 10 + tmp;
            xtmp = xtmp / 10;
        } 

        res = signure * res;
        if res > std::i32::MAX.into() || res < std::i32::MIN.into() {
            res = 0;
        }

        return res as i32;
    }
}

fn main() {
    let mut res = Solution::reverse(123);
    println!("{}", res);

    res = Solution::reverse(-123);
    println!("{}", res);

     res = Solution::reverse(120);
    println!("{}", res);

     res = Solution::reverse(-120);
    println!("{}", res);

    
     res = Solution::reverse(1534236469);
    println!("{}", res);
    // println!("{}", 1534236469 < 2i32.pow(31) - 1);
}