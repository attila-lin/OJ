struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ret = Vec::new();

        if strs.is_empty() {
            return "".into();
        }

        let mut min_len = 1000000;
        for s in &strs {
            let len = s.len();
            if min_len > len {
                min_len = len;
            }
        }

        let mut count = 0;
        loop {
            if min_len <= count {
                break;
            }

            let c = strs[0].chars().nth(count).unwrap();
            let mut success = true;
            for s in &strs {
                if s.chars().nth(count).unwrap() != c {
                    success = false;
                    break;
                }
            }

            if success {
                ret.push(c);
            } else {
                break;
            }

            count += 1;
        }

        return ret.iter().cloned().collect::<String>();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]),
            "fl".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()]),
            "".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["".into(), "".into(), "".into()]),
            "".to_string()
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::longest_common_prefix(vec![]), "".to_string());
    }
}
