struct Solution;


impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut s = x.to_string();

        let test = s.chars().rev().collect::<String>();

        return s == test;

    }
}


fn main() {
    let mut res = Solution::is_palindrome(121);
    println!("{}", res);

    res = Solution::is_palindrome(-121);
    println!("{}", res);

     res = Solution::is_palindrome(10);
    println!("{}", res);

     res = Solution::is_palindrome(0);
    println!("{}", res);

}