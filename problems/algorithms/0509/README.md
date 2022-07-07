# Problem #509 - Fibonacci Number (Easy)

## Problem description

The **Fibonacci numbers**, commonly denoted `F(n)` form a sequence, called the **Fibonacci sequence**, such that each number is the sum of the two preceding ones, starting from `0` and `1`. That is,

```
F(0) = 0, F(1) = 1
F(n) = F(n - 1) + F(n - 2), for n > 1.
```

Given `n`, calculate `F(n)`.

### Example 1:

```
Input: n = 2
Output: 1
Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
```

### Example 2:

```
Input: n = 3
Output: 2
Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
```

### Example 3:

```
Input: n = 4
Output: 3
Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
```

### Constraints:

-   `0 <= n <= 30`

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary>Reveal</summary>
  The most obvious and also almost the best approach to this problem is the same as you would approach it if you were asked to do it on paper. You would probably start with 0 and 1, add them together, get another 1, then do 1+1 and get 2, then 1+2 and get 3, 2+3 to get 5 etc. Now you can just translate it into code - just keep the last 2 numbers computed, add them together to get the next one, and replace the smallest number you are storing with the currently computed one. You stop once you reached the nth number.
  <p>
    
  |         Implementation          | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                    |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :-------------------------------------------------: |
  |              [Rust](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/Rust/solution.rs)               |       O(N)      |       O(1)       | 0 ms, faster than 100.00% of Rust online submissions | 2 MB, less than 91.81% of Rust online submissions |
  |              [C](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/C/solution.c)               |       O(N)      |       O(1)       | 0 ms, faster than 100.00% of C online submissions | 5.3 MB, less than 97.02% of C online submissions |
</details>

## References

[https://leetcode.com/problems/fibonacci-number/](https://leetcode.com/problems/fibonacci-number/)
