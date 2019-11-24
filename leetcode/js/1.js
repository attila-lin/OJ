/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(nums, target) {
    for(var i = 0; i < nums.length; i++){
        var a = nums[i];
        var findFirstLargeNumber = function (element) {
            return element == target - a;
        }
        var j = nums.findIndex(findFirstLargeNumber)
        if( j != -1 && j != i ){
            return [i, j];
        }
    }
};

console.log(twoSum([2, 7, 11, 15], 9));


console.log(twoSum([3,2,4], 6));