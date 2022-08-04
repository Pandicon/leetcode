# Problem #2235 - Add Two Integers (Easy)

## Problem description

Given two integers `num1` and `num2`, return _the **sum** of the two integers_.

### Example 1:

```
Input: num1 = 12, num2 = 5
Output: 17
Explanation: num1 is 12, num2 is 5, and their sum is 12 + 5 = 17, so 17 is returned.
```

### Example 2:

```
Input: num1 = -10, num2 = 4
Output: -6
Explanation: num1 + num2 = -6, so -6 is returned.
```

### Constraints:

-   `-100 <= num1, num2 <= 100`

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary><b>Reveal</b></summary>

This question is most likely there for people that are just getting familiar with leetcode. The obvious solution is to use the built-in addition operator of the language. You could of course try to implement your own way through bit shifts and such, but there is no benefit to that.
Just for the experience I implemented both the built-in solution (named "built-in" in the table below) and the bitwise one (named "bitwise" in the table below) in languages that allowed me to do so (for example C doesn't allow you to left-shift a negative number, unlike Rust).
Note that I mark the built-in implementations as having O(1) time complexity, because modern computers usually add in parallel, which usually allows for O(1) time complexity. However, the bitwise implementation is not parallelised, so it takes O(log(N)) time.

  <p>
    
  |         Implementation          | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                    |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :-------------------------------------------------: |
  |              [Rust (built-in)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2235/Rust/solution_built_in.rs)               |       O(1)      |       O(1)       | 0 ms, faster than 100.00% of Rust online submissions | 2.0 MB, less than 58.79% of Rust online submissions |
  |              [Rust (bitwise)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2235/Rust/solution_bitwise.rs)               |       O(log(N))      |       O(1)       | 0 ms, faster than 100.00% of Rust online submissions | 2.0 MB, less than 58.79% of Rust online submissions |
  |              [C (built-in)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2235/C/solution_built_in.c)               |       O(1)      |       O(1)       | 0 ms, faster than 100.00% of C online submissions | 5.4 MB, less than 70.74% of C online submissions |
  |              [C++ (built-in)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2235/Cpp/solution_built_in.cpp)               |       O(1)      |       O(1)       | 0 ms, faster than 100.00% of C++ online submissions | 5.9 MB, less than 73.36% of C++ online submissions |
  |              [Go (built-in)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2235/Go/solution_built_in.go)               |       O(1)      |       O(1)       | 0 ms, faster than 100.00% of Go online submissions | 1.9 MB, less than 78.48% of Go online submissions |
  |              [Go (bitwise)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2235/Go/solution_bitwise.go)               |       O(log(N))      |       O(1)       | 0 ms, faster than 100.00% of Go online submissions | 2.0 MB, less than 78.48% of Go online submissions |
  |              [Swift (built-in)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2235/Swift/solution_built_in.swift)               |       O(1)      |       O(1)       | 0 ms, faster than 100.00% of Swift online submissions | 13.4 MB, less than 82.28% of Swift online submissions |
  |              [Swift (bitwise)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2235/Swift/solution_bitwise.swift)               |       O(log(N))      |       O(1)       | 0 ms, faster than 100.00% of Swift online submissions | 13.7 MB, less than 50.38% of Swift online submissions |
</details>

## References

[https://leetcode.com/problems/add-two-integers/](https://leetcode.com/problems/add-two-integers/)
