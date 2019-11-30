#[macro_use]
use crate::common::linked_list::{ListNode, to_list};

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            return None;
        }

        let mut ll2 = l2.clone();
        let mut ll1 = l1.clone();

        loop {
            let c1 = if let Some(cur_1) = &ll1 {
                Some(cur_1.val)
            } else {
                None
            };

            let c2 = if let Some(cur_2) = &ll2 {
                Some(cur_2.val)
            } else {
                None
            };

            if c1.is_none() || c2.is_none() {
                break;
            }

            ll1 = if let Some(cur_1) = &ll1 {
                cur_1.next
            } else {
                None
            };
            ll2 = if let Some(cur_2) = &ll2 {
                cur_2.next
            } else {
                None
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(
            Solution::merge_two_lists(None, to_list(vec![1, 3, 4])),
            to_list(vec![1, 3, 4])
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }
}
