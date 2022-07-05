# Problem #167 - Two Sum II - Input Array Is Sorted (Medium)

## Problem description

Given a **1-indexed** array of integers `numbers` that is already _**sorted in non-decreasing order**_, find two numbers such that they add up to a specific `target` number. Let these two numbers be `numbers[index₁]` and `numbers[index₂]` where `1 <= index₁ < index₂ <= numbers.length`.

Return _the indices of the two numbers,_ `index₁` _and_ `index₂`_, **added by one** as an integer array_ `[index₁, index₂]` _of length 2._

The tests are generated such that there is **exactly one solution**. You **may not** use the same element twice.

Your solution must use only constant extra space.

### Example 1:

```
Input: numbers = [2,7,11,15], target = 9
Output: [1,2]
Explanation: The sum of 2 and 7 is 9. Therefore, index₁ = 1, index₂ = 2. We return [1, 2].
```

### Example 2:

```
Input: numbers = [2,3,4], target = 6
Output: [1,3]
Explanation: The sum of 2 and 4 is 6. Therefore index₁ = 1, index₂ = 3. We return [1, 3].
```

### Example 3:

```
Input: numbers = [-1,0], target = -1
Output: [1,2]
Explanation: The sum of -1 and 0 is -1. Therefore index₁ = 1, index₂ = 2. We return [1, 2].
```

### Constraints:

-   `2 <= numbers.length <= 3 * 10⁴`
-   `-1000 <= numbers[i] <= 1000`
-   `numbers` is sorted in **non-decreasing order**.
-   `-1000 <= target <= 1000`
-   The tests are generated such that there is **exactly one solution**.

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary><b>Reveal</b></summary>

This problem is basically the same as the [Problem #1 - Two Sum](https://leetcode.com/problems/two-sum/) ([my solutions](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0001)), but we don't have to deal with sorting the array, as it is already sorted when it is handed out to us. The "issue" is the requirement to only use constant extra space, that eliminates the hashmap solution, since in the worst case you will get all numbers apart from one into the hashmap, which is O(N). This leaves us with the two pointers approach, where you initialise two pointers, one at the start of the array and one at the end. Then you loop while the first pointer is pointing at a lower index than the second one (since once they switch, you would be checking the same pairs as before). Each iteration you check the sum of the numbers that the pointers are pointing at. If the sum is equal to the target number, you return the pointers added by one and you are done. If it is lower than the target, you add one to the first pointer, moving it one value up, making it point at a higher value (since the array is sorted in ascending order, so `arr[i] <= arr[i+1]`), and therefore increasing the total sum. If the sum is greater than the target, you subtract one from the second pointer, moving it one value down, making it point at a lower value, and therefore decreasing the total sum. If you don't find a pair that adds up to the target, you know there isn't one, since you had to visit all the pairs in the array (you were only moving by one, so you couldn't skip any value).

  <p>
    
  |         Implementation          | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                    |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :-------------------------------------------------: |
  |              [Rust](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0167/solution.rs)               |       O(N)      |       O(1)       | 0 ms, faster than 100.00% of Rust online submissions | 2.1 MB, less than 100.00% of Rust online submissions |
</details>

## References

[https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)
