# Problem #376 - Wiggle Subsequence (Medium)

## Problem description

A **wiggle sequence** is a sequence where the differences between successive numbers strictly alternate between positive and negative. The first difference (if one exists) may be either positive or negative. A sequence with one element and a sequence with two non-equal elements are trivially wiggle sequences.

-   For example, `[1, 7, 4, 9, 2, 5]` is a **wiggle sequence** because the differences `(6, -3, 5, -7, 3)` alternate between positive and negative.
-   In contrast, `[1, 4, 7, 2, 5]` and `[1, 7, 4, 5, 5]` are not wiggle sequences. The first is not because its first two differences are positive, and the second is not because its last difference is zero.

A **subsequence** is obtained by deleting some elements (possibly zero) from the original sequence, leaving the remaining elements in their original order.
Given an integer array `nums`, return \*the length of the longest **wiggle subsequence\*** of `nums`.

**Follow up:** Could you solve this in `O(n)` time?

### Example 1:

```
Input: nums = [1,7,4,9,2,5]
Output: 6
Explanation: The entire sequence is a wiggle sequence with differences (6, -3, 5, -7, 3).
```

### Example 2:

```
Input: nums = [1,17,5,10,13,15,10,5,16,8]
Output: 7
Explanation: There are several subsequences that achieve this length.
One is [1, 17, 10, 13, 10, 16, 8] with differences (16, -7, 3, -3, 6, -8).
```

### Example 3:

```
Input: nums = [1,2,3,4,5,6,7,8,9]
Output: 2
```

### Constraints:

-   `1 <= nums.length <= 1000`
-   `0 <= nums[i] <= 1000`

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary>Reveal</summary>
  The problem wants you to find a sequence of numbers where the numbers are peaks and valleys, which you can achieve by ignoring numbers that are not peaks or valleys.
  My implementation keeps track of the current "direction" of the numbers, 1 for ascending and -1 for descending.
  Then given two numbers, if their direction is opposite from the current one, you know the direction changed, so you found a peak/valley.
  That also means you found a value that will be in the final subsequence, so you can add 1 to the current maximum length.
  <p>
    
  | Time complexity | Space complexity | Primary implementation language |                        Runtime                       |                     Memory Usage                    |
  | :-------------: | :--------------: | :------------------------------:| :--------------------------------------------------: | :-------------------------------------------------: |
  |       O(N)      |       O(1)       |              Rust               | 0 ms, faster than 100.00% of Rust online submissions | 2.2 MB, less than 23.08% of Rust online submissions |
</details>

## References

[https://leetcode.com/problems/wiggle-subsequence/](https://leetcode.com/problems/wiggle-subsequence/)

[https://leetcode.com/problems/wiggle-subsequence/discuss/2229700/C%2B%2B-oror-Easy-Solution-oror-Full-Explanation](https://leetcode.com/problems/wiggle-subsequence/discuss/2229700/C%2B%2B-oror-Easy-Solution-oror-Full-Explanation) - An awesome explanation of a different approach
