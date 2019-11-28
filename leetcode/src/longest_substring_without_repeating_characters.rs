struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();

        let chars: Vec<char> = s.chars().collect();
        let n = chars.len() as i32;
        if n == 0 || n == 1 {
            return n;
        }

        let mut res: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;

        while i < n && j < n {
            if !set.contains(&chars[j as usize]) {
                set.insert(chars[j as usize]);
                j += 1;
                res = std::cmp::max(res, j - i);
            } else {
                set.remove(&chars[i as usize]);
                i += 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbbbbbbbbbbbb".into()),
            1
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
    }
}
