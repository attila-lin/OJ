// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

#[macro_use]
use crate::common::linked_list::{ListNode, to_list};

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let num_1 = Self::get_number_by_list(l1);
        let num_2 = Self::get_number_by_list(l2);

        let res = num_1 + num_2;
        // dbg!(&res, &num_1, &num_2);

        Self::gen_list_by_number(res)
    }

    fn get_number_by_list(l: Option<Box<ListNode>>) -> i128 {
        if let Some(mut list) = l {
            let mut res = 0;
            let mut pos = 0;
            loop {
                res = res + list.val as i128 * 10_i128.pow(pos);
                if list.next == None {
                    break;
                }

                pos += 1;

                list = list.next.as_ref().unwrap().to_owned();
            }

            return res;
        } else {
            return 0;
        }
    }

    fn gen_list_by_number(mut val: i128) -> Option<Box<ListNode>> {
        let mut v = vec![];
        loop {
            v.push(val % 10);
            val /= 10;
            if val == 0 {
                break;
            }
        }

        v.reverse();

        let mut ret = ListNode::new(v[0] as i32);
        let mut is_first = true;
        for _v in v {
            let tmp = if !is_first {
                Some(ret.clone())
            } else {
                is_first = false;
                continue;
            };

            ret = ListNode::new(_v as i32);
            if !tmp.is_none() {
                ret.next = Some(Box::new(tmp.unwrap()));
            }
        }
        Some(Box::new(ret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![1, 8]), to_list(vec![0])),
            to_list(vec![1, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(
                to_list(vec![9]),
                to_list(vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9])
            ),
            to_list(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(
                to_list(vec![
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 1
                ]),
                to_list(vec![5, 6, 4])
            ),
            to_list(vec![
                6, 6, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1
            ])
        );
    }
}
