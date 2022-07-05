# Problem #2149 - Rearrange Array Elements by Sign (Medium)

## Problem description

You are given a **0-indexed** integer array `nums` of **even** length consisting of an **equal** number of positive and negative integers.

You should **rearrange** the elements of `nums` such that the modified array follows the given conditions:

1. Every **consecutive pair** of integers have **opposite signs**.
2. For all integers with the same sign, the **order** in which they were present in `nums` is **preserved**.
3. The rearranged array begins with a positive integer.

Return _the modified array after rearranging the elements to satisfy the aforementioned conditions_.

### Example 1:

```
Input: nums = [3,1,-2,-5,2,-4]
Output: [3,-2,1,-5,2,-4]
Explanation:
The positive integers in nums are [3,1,2]. The negative integers are [-2,-5,-4].
The only possible way to rearrange them such that they satisfy all conditions is [3,-2,1,-5,2,-4].
Other ways such as [1,-2,2,-5,3,-4], [3,1,2,-2,-5,-4], [-2,3,-5,1,-4,2] are incorrect because they do not satisfy one or more conditions.
```

### Example 2:

```
Input: nums = [-1,1]
Output: [1,-1]
Explanation:
1 is the only positive integer and -1 the only negative integer in nums.
So nums is rearranged to [1,-1].
```

### Constraints:

-   `2 <= nums.length <= 2 * 10⁵`
-   `nums.length` is **even**
-   `1 <= |nums[i]| <= 10⁵`
-   `nums` consists of **equal** number of positive and negative integers.

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary><b>Reveal</b></summary>
  There are two approaches I will cover here, both of them should be very similar in terms of performance and memory usage (they both have the same time and space complexity).
  <p>
    
  <details>
  <summary>First approach</summary>

This approach is the more obvious one in my opinion since I thought of it almost immediately after reading the problem. You can basically go through the given array and filter the elements into two new arrays. One of them will hold positive numbers, while the other one will hold negative numbers. Then you can push the numbers into the result array, altering between a number from the array holding the positive numbers and from the array holding the negative numbers. This way you start with a positive number and the numbers will be altering between positive and negative. _This approach is named as the "filtering method" in the table below._

  </details>
    
  <details>
  <summary>Second approach</summary>

You can also create an array of 0s (or any other number really) of the same length as the numbers array given to you. Then you can create two pointers - one for the index where the next positive number will go (set to 0), and one for the index of the next negative number (set to 1). Then you can loop through the array of numbers and if you encounter a positive number, put it on the index saved in the positive number pointer and then increase that pointer by two. If you encounter a negative number, do the same with the negative number pointer. This way you will also have an altering sequence of positive and negative numbers (the positive pointer will go 0 -> 2 -> 4... and the negative 1 -> 3 -> 5...) and the final array will also start with a positive number (since the positive number pointer is initialised to 0). _This approach is named as "two pointers" in the table below._

  </details>

  <p>
    
  |          Implementation         | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                     |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :--------------------------------------------------: |
  |     [Rust (filtering method)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2149/Rust/solution_filtering.rs)     |       O(N)      |       O(N)       | 76 ms, faster than 93.75% of Rust online submissions | 4.6 MB, less than 100.00% of Rust online submissions |
  |       [Rust (two pointers)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2149/Rust/solution_two_pointers.rs)      |       O(N)      |       O(N)       | 72 ms, faster than 93.75% of Rust online submissions |  5.4 MB, less than 31.25% of Rust online submissions |
</details>

## References

[https://leetcode.com/problems/rearrange-array-elements-by-sign/](https://leetcode.com/problems/rearrange-array-elements-by-sign/)
