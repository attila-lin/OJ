struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let ss = s.trim();

        let s_list = ss.split(" ").collect::<Vec<&str>>();
        if s_list.len() == 0 {
            return 0;
        } else {
            let index = s_list.len() - 1;
            s_list[index as usize].len() as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::length_of_last_word("Hello World".into()), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::length_of_last_word(" ".into()), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_last_word("   a ".into()), 1);
    }
}
