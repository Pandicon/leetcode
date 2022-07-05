# Problem #922 - Sort Array By Parity II (Easy)

## Problem description

Given an array of integers `nums`, half of the integers in `nums` are **odd**, and the other half are **even**.
Sort the array so that whenever `nums[i]` is odd, `i` is **odd**, and whenever `nums[i]` is even, `i` is **even**.
Return _any answer array that satisfies this condition_.
**Follow Up:** Could you solve it in-place?

### Example 1:

```
Input: nums = [4,2,5,7]
Output: [4,5,2,7]
Explanation: [4,7,2,5], [2,5,4,7], [2,7,4,5] would also have been accepted.
```

### Example 2:

```
Input: nums = [2,3]
Output: [2,3]
```

### Constraints:

-   `2 <= nums.length <= 2 * 10⁴`
-   `nums.length` is even.
-   Half of the integers in `nums` are even.
-   `0 <= nums[i] <= 1000`

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary><b>Reveal</b></summary>
  There are four approaches I will cover here, all of them should be very similar in terms of performance and memory usage (they have the same time and space complexity).
  <p>
  <details>
  <summary><b>First approach</b></summary>

This approach is the more obvious one in my opinion since I thought of it almost immediately after reading the problem. You can basically go through the given array and filter the elements into two new arrays. One of them will hold even numbers, while the other one will hold odd numbers. Then you can push the numbers into the result array, altering between a number from the array holding the even numbers and from the array holding the odd numbers. This way you start with an even number and the numbers will be altering between even and odd. _This approach is named "filtering method" in the table below._

  </details>
  <details>
  <summary><b>Second approach</b></summary>

You can also create an array of 0s (or any other number really) of the same length as the numbers array given to you. Then you can loop through the array and if the parity of the index is different from the parity of the number, then you hit an invalid placement. You then check the stack variable if there is a value of the opposite parity waiting to be placed. If there is, switch the indexes of the value found on the stack and of the current value and you are done. If there is no value to be switched with the current one, just push the current value and index into the corresponding place of the stack for it to be used later. _This approach is named "stack" in the table below._

  </details>
  <details>
  <summary><b>Third approach</b></summary>

This approach is basically the same as the previous one, but instead of creating an array of 0s, you directly mutate the input array, which should save you some memory and initialisation time. This also solves the follow up, that's why I didn't mention it in the second approach if someone wanted to discover it themselves. _This approach is named "mutable input" in the table below._

  </details>
  <details>
  <summary><b>Fourth approach</b></summary>

This approach is once again makes use of mutable access to the input array, but it is not required (you will have to initialise the array of 0s, which requires more memory and time). You initialise two pointers, one pointing at the index 0, and another pointing at the index 1. Then you loop while both of the indexes are within the array. In each itereation, you switch the values at the two indexes, then you loop while the first pointer is pointing at an even value and is within the array, increasing the pointer by two each time. Then once you exit that loop, you do the same with the other pointer, but while it is pointing at an odd value. Once you exit that loop too, it is guaranteed that you either moved out of the array or the first pointer (with an even value) is pointing at an odd number and the second pointer (with an odd value) is pointing at an even value. If you moved out of the array, the main loop will break, and you will return the mutated array. If not, you will enter the next iteration, swapping the values at the indexes pointed to by the pointers. As long as you make use of the mutability of the input, this can have constant space complexity. _This approach is named "two pointers" in the table below._

  </details>
  <p>
    
  |          Implementation         | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                     |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :--------------------------------------------------: |
  |     [Rust (filtering method)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0922/Rust/solution_filtering.rs)     |       O(N)      |       O(N)       | 9 ms, faster than 100.00% of Rust online submissions | 2.5 MB, less than 42.86% of Rust online submissions |
  |       [Rust (stack)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0922/Rust/solution_stack.rs)      |       O(N)      |       O(N)       | 7 ms, faster than 100.00% of Rust online submissions |  2.2 MB, less than 100.00% of Rust online submissions |
  |       [Rust (mutable input)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0922/Rust/solution_mutable_input.rs)      |       O(N)      |       O(N)       | 9 ms, faster than 100.00% of Rust online submissions |  2.2 MB, less than 100.00% of Rust online submissions |
  |       [Rust (two pointers)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0922/Rust/solution_two_pointers.rs)      |       O(N)      |       O(1)       | 6 ms, faster than 100.00% of Rust online submissions |  2.1 MB, less than 100.00% of Rust online submissions |
</details>

## References

[https://leetcode.com/problems/sort-array-by-parity-ii/](https://leetcode.com/problems/sort-array-by-parity-ii/)
