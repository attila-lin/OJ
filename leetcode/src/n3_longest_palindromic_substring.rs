pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len() as i32;

        if n == 0 {return "".into();}

        let mut start = 0;
        let mut end = 0;
        let mut i = 0;
        while i < n {
            // 以i为中心
            let len1 = Self::get_max(&chars, i, i);
            let len2 = Self::get_max(&chars, i, i + 1);

            let len = std::cmp::max(len1, len2);
            if len > end - start {
                start = i - (len - 1) / 2;
                end = i + len / 2;
            }

            i += 1;
        }

        chars[(start as usize)..(end as usize + 1)].iter().collect()
    }

    fn get_max(chars: &Vec<char>, mut left: i32, mut right: i32) -> i32 {
        let mut s = 0;
        loop {
            left = left - s;
            right = right + s;
            if left < 0 || right >= chars.len() as i32 {
                break;
            }

            if chars[left as usize] != chars[right as usize] {
                break;
            }

            s += 1;
        }

        right - 1 - (left + 1) -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_palindrome("babad".into()),
            "aba".to_string()
        );
    }


    fn test_2() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".into()),
            "bb".to_string()
        );
    }

    fn test_3() {
        assert_eq!(
            Solution::longest_palindrome("babadada".into()),
            "adada".to_string()
        );
    }
}
