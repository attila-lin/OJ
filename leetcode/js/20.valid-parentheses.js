/*
 * @lc app=leetcode id=20 lang=javascript
 *
 * [20] Valid Parentheses
 *
 * https://leetcode.com/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (35.77%)
 * Total Accepted:    506.8K
 * Total Submissions: 1.4M
 * Testcase Example:  '"()"'
 *
 * Given a string containing just the characters '(', ')', '{', '}', '[' and
 * ']', determine if the input string is valid.
 * 
 * An input string is valid if:
 * 
 * 
 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
 * 
 * 
 * Note that an empty string is also considered valid.
 * 
 * Example 1:
 * 
 * 
 * Input: "()"
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "()[]{}"
 * Output: true
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: "(]"
 * Output: false
 * 
 * 
 * Example 4:
 * 
 * 
 * Input: "([)]"
 * Output: false
 * 
 * 
 * Example 5:
 * 
 * 
 * Input: "{[]}"
 * Output: true
 * 
 * 
 */
/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function(s) {
    var stake = [];
    for(var i = 0; i < s.length; i ++) {
        var c = s[i];
        if(c == '(' || c == '{' || c == '[') {
            stake.push( c );
        }
        
        if(c == ')' ) {
            if(stake.length > 0 && stake[stake.length - 1] == '(')
                stake.pop();
            else
                return false;
        }
        else if(c == '}') {
            if( stake.length > 0 && stake[stake.length - 1] == '{')
            stake.pop();
            else
                return false;
        }
        else if(c == ']' ) {
            if(stake.length > 0 && stake[stake.length - 1] == '[')
            stake.pop();
            else{
                return false;
            }
        }
    }

    return stake.length == 0;
};
