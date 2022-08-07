# Problem #2161 - Rearrange Array Elements by Sign (Medium)

## Problem description

You are given a **0-indexed** integer array `nums` and an integer `pivot`. Rearrange `nums` such that the following conditions are satisfied:

-   Every element less than `pivot` appears **before** every element greater than `pivot`.
-   Every element equal to `pivot` appears **in between** the elements less than and greater than `pivot`.
-   The **relative order** of the elements less than `pivot` and the elements greater than `pivot` is maintained.
    -   More formally, consider every `pᵢ`, `pⱼ ` where `pᵢ` is the new position of the `iᵗʰ` element and `pⱼ ` is the new position of the `jᵗʰ` element. For elements less than `pivot`, if `i < j` and `nums[i] < pivot` and `nums[j] < pivot`, then `pᵢ < pⱼ `. Similarly for elements greater than `pivot`, if `i < j` and `nums[i] > pivot` and `nums[j] > pivot`, then `pᵢ < pⱼ `.

Return `nums` _after the rearrangement._

### Example 1:

```
Input: nums = [9,12,5,10,14,3,10], pivot = 10
Output: [9,5,3,10,10,12,14]
Explanation:
The elements 9, 5, and 3 are less than the pivot so they are on the left side of the array.
The elements 12 and 14 are greater than the pivot so they are on the right side of the array.
The relative ordering of the elements less than and greater than pivot is also maintained. [9, 5, 3] and [12, 14] are the respective orderings.
```

### Example 2:

```
Input: nums = [-3,4,3,2], pivot = 2
Output: [-3,2,4,3]
Explanation:
The element -3 is less than the pivot so it is on the left side of the array.
The elements 4 and 3 are greater than the pivot so they are on the right side of the array.
The relative ordering of the elements less than and greater than pivot is also maintained. [-3] and [4, 3] are the respective orderings.
```

### Constraints:

-   `1 <= nums.length <= 10⁵`
-   `-10⁶ <= nums[i] <= 10⁶`
-   `pivot` equals to an element of `nums`.

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary><b>Reveal</b></summary>
  There are two approaches I will cover here, both of them should be very similar in terms of performance and memory usage (they both have the same time and space complexity).
  <p>
    
  <details>
  <summary>First approach</summary>

This approach is the more obvious one in my opinion. You can go through the array, push the numbers lower than the pivot, count how many times there is a number equal to the pivot, and push numbers greater than the pivot into a separate array. Then you push the pivot into the result array the number of times you counted it during the initial traversal and then you append the elements from the array storing the values higher than the pivot onto the result. _This approach is named as "filtering" in the table below._

  </details>
    
  <details>
  <summary>Second approach</summary>

You can also create an array of 0s (or any other number really) of the same length as the numbers array given to you. Then you will count how many elements are lower and how many are equal to the pivot given to you. Then you traverse the array again, appending the lower elements from the 0th index, the equal ones from the index of the value of how many elements were lower than the pivot (so at the end of the part where lower numbers will be stored), and higher ones to the index of the value of how many elements were lower or equal to the pivot (so at the end of the part where equal numbers will be stored). You increase each of the pointers when you encounter a number referring to it. _This approach is named as "three pointers" in the table below._

  </details>

  <p>
    
  |          Implementation         | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                     |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :--------------------------------------------------: |
  |     [Rust (filtering)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2161/Rust/solution_filtering.rs)     |       O(N)      |       O(N)       | 75 ms, faster than 100.00% of Rust online submissions | 4.1 MB, less than 40.00% of Rust online submissions |
  |       [Rust (three pointers)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2161/Rust/solution_three_pointers.rs)      |       O(N)      |       O(N)       | 98 ms, faster than 53.33% of Rust online submissions |  3.7 MB, less than 80.00% of Rust online submissions |
  |     [C (filtering)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2161/C/solution_filtering.c)     |       O(N)      |       O(N)       | 875 ms, faster than 89.47% of C online submissions | 93.9 MB, less than 71.05% of C online submissions |
  |       [C (three pointers)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2161/C/solution_three_pointers.c)      |       O(N)      |       O(N)       | 880 ms, faster than 89.47% of C online submissions |  93.8 MB, less than 71.05% of C online submissions |
  |     [C++ (filtering)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2161/Cpp/solution_filtering.cpp)     |       O(N)      |       O(N)       | 219 ms, faster than 97.72% of C++ online submissions | 130.3 MB, less than 37.53% of C++ online submissions |
  |       [C++ (three pointers)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2161/Cpp/solution_three_pointers.cpp)      |       O(N)      |       O(N)       | 253 ms, faster than 82.23% of C++ online submissions |  123.2 MB, less than 93.75% of C++ online submissions |
</details>

## References

[https://leetcode.com/problems/partition-array-according-to-given-pivot/](https://leetcode.com/problems/partition-array-according-to-given-pivot/)
