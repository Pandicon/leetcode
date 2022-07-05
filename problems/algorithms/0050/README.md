# Problem #50 - Pow(x, n) (Medium)

## Problem description

Implement [pow(x, n)](https://cplusplus.com/reference/valarray/pow/), which calculates `x` raised to the power `n` (i.e., `xⁿ`).

### Example 1:

```
Input: x = 2.00000, n = 10
Output: 1024.00000
```

### Example 2:

```
Input: x = 2.10000, n = 3
Output: 9.26100
```

### Example 3:

```
Input: x = 2.00000, n = -2
Output: 0.25000
Explanation: 2⁻² = 1/2² = 1/4 = 0.25
```

### Constraints:

-   `-100.0 < x < 100.0`
-   `-2³¹ <= n <= 2³¹-1`
-   `-10⁴ <= xⁿ <= 10⁴`

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary><b>Reveal</b></summary>

The first thing most people would probably try is multiplying the base n times, which is a good thought. Sadly the exponent here can be up to 2³¹, which eliminates this approach since it would take too long. If you try it, LeetCode will give you a "Time Limit Exceeded" error.
The better way here is to use the "Square and multiply" algorithm, which is brilliantly explained by [Computerphile](https://www.youtube.com/watch?v=cbGB__V8MNk). It lets you use the fact that `x²ⁿ = (xⁿ)²`, so instead of going from `xⁿ` to `x²ⁿ` with n multiplications, you use just one squaring, bringing the time complexity from `n` to `log(n)`.

You can then either take the "bitwise" approach, where you check the individual bits as it is explained in the video linked above, or the "division" approach, where you just chop-off parts of the exponent gradually. Both of the approaches should be roughly the same in terms of performance and memory usage.

  <p>
    
  |         Implementation          | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                    |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :-------------------------------------------------: |
  |              [Rust (bitwise)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0050/Rust/solution_bitwise.rs)               |       O(log(N))      |       O(1)       | 0 ms, faster than 100.00% of Rust online submissions | 1.9 MB, less than 95.15% of Rust online submissions |
  |              [Rust (division)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0050/Rust/solution_division.rs)               |       O(log(N))      |       O(1)       | 0 ms, faster than 100.00% of Rust online submissions | 2 MB, less than 66.02% of Rust online submissions |
  |              [C (bitwise)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0050/C/solution_bitwise.c)               |       O(log(N))      |       O(1)       | 0 ms, faster than 100.00% of C online submissions | 5.5 MB, less than 85.51% of C online submissions |
  |              [C (division)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0050/C/solution_division.c)               |       O(log(N))      |       O(1)       | 0 ms, faster than 100.00% of C online submissions | 5.4 MB, less than 85.51% of C online submissions |
</details>

## References

[https://leetcode.com/problems/powx-n/](https://leetcode.com/problems/powx-n/)
