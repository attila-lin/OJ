mod common;

mod longest_substring_without_repeating_characters;
mod n0002_add_two_numbers;
mod n14_longest_common_prefix;
mod n21_merge_two_lists;
mod n3_longest_palindromic_substring;
mod n58_length_of_last_word;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    println!("Hello, world!");

    let res = n3_longest_palindromic_substring::Solution::longest_palindrome("babad".into());
}
