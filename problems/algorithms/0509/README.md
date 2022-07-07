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
  <summary><b>Reveal</b></summary>
  There are six approaches I will cover here, each of them will have different performance.

  <details>
  <summary>First approach</summary>

Another approach is to calculate the number recursively, so by calculating the number from the top by calling the function on an index lower by one and by two and adding the results. You just need to make sure you return 0 and 1 for indexes 0 and 1, else it will never terminate. The space complexity is O(N) if we take into account the stack calls, else it would be O(1). The main issue with this approach is that you calculate each number multiple times (3rd number requires 1st and 2nd, which requires 1st etc.), which is reflected in the O(2ᴺ) time complexity. _This approach is named "recursive" in the table below_

  </details>

  <details>
  <summary>Second approach</summary>

This is an improvement of the second approach by using memoization. We can save the previously calculated values and return them, instead of calculating them again, pushing this back to O(N) time complexity. _This approach is named "recursive with memoization" in the table below_

  </details>
  <details>
  <summary>Third approach</summary>

This approach calculates the number from the bottom, saving the previous results in an array and using them for the next values. _This approach is named "iterative without forgetting" in the table below_

  </details>
  <details>
  <summary>Fourth approach</summary>

The most obvious and also almost the best approach to this problem is the same as you would approach it if you were asked to do it on paper. You would probably start with 0 and 1, add them together, get another 1, then do 1+1 and get 2, then 1+2 and get 3, 2+3 to get 5 etc. and you probably wouldn't really care about the previous results, since you don't need them. Now you can just translate it into code - just keep the last 2 numbers computed, add them together to get the next one, and replace the smallest number you are storing with the currently computed one. You stop once you reached the nth number. _This approach is named "iterative" in the table below_

  </details>
  <details>
  <summary>Fifth approach</summary>

You can make use of a formula for calculating the fibonacci numbers: `F(n) = round((Φⁿ - (1 - Φ)ⁿ)/√5)`. If you can not use built-in functions like the power one, check out [my explanations of problem #50](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0050/), which focuses on this. _This approach is named "formula" in the table below_

  </details>
  <details>
  <summary>Sixth approach</summary>

You can optimise the formula quite well, since 1 - Φ is around -0.618, which gets small pretty fast. The optimised formula is `F(n) = round(Φⁿ/√5)`. If you had to also support negative numbers, you would use `F(n) = round((1 - Φ)ⁿ/√5)` for them, since `x⁻ⁿ = (1/x)ⁿ` and `1/(1 - Φ) = -Φ`, that part would get much bigger than `Φ⁻ⁿ`, which is smaller than 1, quite fast. Once again if you can not use built-in functions like the power one, check out [my explanations of problem #50](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0050/), which focuses on this. _This approach is named "optimised formula" in the table below_

  </details>
  <p>
    
  |         Implementation          | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                    |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :-------------------------------------------------: |
  |              [Rust (recursive)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/Rust/solution_recursive.rs)               |       O(2ᴺ)      |       O(N)       | 9 ms, faster than 12.87% of Rust online submissions | 2.1 MB, less than 53.22% of Rust online submissions |
  |              [Rust (recursive with memoization)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/Rust/solution_recursive_memoization.rs)               |       O(N)      |       O(N)       | 0 ms, faster than 100.00% of Rust online submissions | 2.1 MB, less than 53.22% of Rust online submissions |
  |              [Rust (iterative without forgetting)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/Rust/solution_iterative_no_forgetting.rs)               |       O(N)      |       O(N)       | 0 ms, faster than 100.00% of Rust online submissions | 2.1 MB, less than 53.22% of Rust online submissions |
  |              [Rust (iterative)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/Rust/solution_iterative.rs)               |       O(N)      |       O(1)       | 0 ms, faster than 100.00% of Rust online submissions | 2 MB, less than 91.81% of Rust online submissions |
  |              [Rust (formula)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/Rust/solution_formula.rs)               |       O(log(N))      |       O(1)       | 0 ms, faster than 100.00% of Rust online submissions | 2.4 MB, less than 7.60% of Rust online submissions |
  |              [Rust (optimised formula)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/Rust/solution_formula_optimised.rs)               |       O(log(N))      |       O(1)       | 0 ms, faster than 100.00% of Rust online submissions | 2.4 MB, less than 7.60% of Rust online submissions |
  |              [C (recursive)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/C/solution_recursive.c)               |       O(2ᴺ)      |       O(N)       | 17 ms, faster than 26.23% of C online submissions | 5.4 MB, less than 96.84% of C online submissions |
  |              [C (recursive with memoization)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/C/solution_recursive_memoization.c)               |       O(N)      |       O(N)       | 0 ms, faster than 100.00% of C online submissions | 5.4 MB, less than 96.84% of C online submissions |
  |              [C (iterative without forgetting)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/C/solution_iterative_no_forgetting.c)               |       O(N)      |       O(N)       | 0 ms, faster than 100.00% of C online submissions | 5.5 MB, less than 65.95% of C online submissions |
  |              [C (iterative)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/C/solution_iterative.c)               |       O(N)      |       O(1)       | 0 ms, faster than 100.00% of C online submissions | 5.3 MB, less than 97.02% of C online submissions |
  |              [C (formula)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/C/solution_formula.c)               |       O(log(N))      |       O(1)       | 0 ms, faster than 100.00% of C online submissions | 5.6 MB, less than 39.30% of C online submissions |
  |              [C (optimised formula)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0509/C/solution_formula_optimised.c)               |       O(log(N))      |       O(1)       | 0 ms, faster than 100.00% of C online submissions | 5.6 MB, less than 39.30% of C online submissions |
</details>

## References

[https://leetcode.com/problems/fibonacci-number/](https://leetcode.com/problems/fibonacci-number/)
