/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var addTwoNumbers = function(l1, l2) {
    var get_number = function(node) {
        var ret = node.value;
        var count = 1;
        while(node.next){
            node = node.next;
            ret += node.value * pow(10, count);
            count ++;
        }
        return ret;
    }
    
    var total = get_number(l1) + get_number(l2);
    
    while(Math.floor(total) != 0){
        
    }
};

