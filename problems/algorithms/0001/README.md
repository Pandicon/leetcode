# Problem #1 - Two Sum (Easy)

## Problem description

Given an array of integers `nums` and an integer `target`, return _indices of the two numbers such that they add up to `target`_.
You may assume that each input would have **_exactly_ one solution**, and you may not use the _same_ element twice.
You can return the answer in any order.

### Example 1:

```
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
```

### Example 2:

```
Input: nums = [3,2,4], target = 6
Output: [1,2]
```

### Example 3:

```
Input: nums = [3,3], target = 6
Output: [0,1]
```

### Constraints:

-   `2 <= nums.length <= 10⁴`
-   `-10⁹ <= nums[i] <= 10⁹`
-   `-10⁹ <= target <= 10⁹`

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary>Reveal</summary>
  There are three approaches I will cover here, each of them will have different performance.
  <p>
    
**First approach:** The most obvious approach is to just brute-force this. You can have two loops inside each other, running through the indexes of the array, and checking for the sum of the numbers at the indexes to be the target. If they are, return the two indexes. _This approach is named "bruteforce" in the table below)_

**Second approach:** This approach requires sorting the array. This would be easier if you had to return the numbers that add up to the result, but you have to return the indexes of the numbers, so you have to sort the array of number-index pairs (you can sort it into ascending or descending order, but my explanation will be about ascending). Then you initialise two pointers, one at the position 0, and one at the last index of the array. Then you loop while the first pointer is lower than the second one (once they pass around each other, it is useless to continue since you would check the numbers again). Then you check the sum of the numbers at the indexes pointed to by the first and second pointer. If the sum is the target, you return the indexes (not the pointers, since they point to the sorted array, which is different from the original one, you will have to return the indexes stored next to the numbers). If the sum is lower, you increase the first pointer, since that way you will move it up, so onto a higher number (the array is sorted in ascending order, so arr[n] <= arr[n+1]), increasing the total sum. If the sum is higher than the target, you decrease the second pointer, moving it onto a lower number, decreasing the total sum. This way you have to find a solution if one exists. Due to the constrains we don't have to worry about returning the first solution, since there is always only one solution. _This approach is named "sorted" in the table below_.

**Third approach:** This approach is the only linear one here. You loop through the values of the array, and always calculate the difference between the target and the current number. Then you check if the difference is present as a key in the hashmap. If it isn't, you insert the index of the current number with the key being the current number. If it is, you take the value saved under that key, which is the index of the number needed to make the target sum with the current number, and return it along with the current index. _This approach is named "hashmap" in the table below_

  <p>
    
  |          Implementation         | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                     |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :--------------------------------------------------: |
  |     [Rust (bruteforce)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0001/Rust/solution_bruteforce.rs)     |       O(N²)      |       O(1)       | 61 ms, faster than 11.87% of Rust online submissions | 2.2 MB, less than 69.80% of Rust online submissions |
  |     [Rust (sorted)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0001/Rust/solution_sorted.rs)     |       O(N*log(N))      |       O(N)       | 0 ms, faster than 100.00% of Rust online submissions | 2.2 MB, less than 69.80% of Rust online submissions |
  |     [Rust (hashmap)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0001/Rust/solution_hashmap.rs)     |       O(N)      |       O(N)       | 3 ms, faster than 68.63% of Rust online submissions | 2.5 MB, less than 17.01% of Rust online submissions |
</details>

## References

[https://leetcode.com/problems/two-sum/](https://leetcode.com/problems/two-sum/)
