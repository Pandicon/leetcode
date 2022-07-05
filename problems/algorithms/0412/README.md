# Problem #412 - Fizz Buzz (Easy)

## Problem description

Given an integer `n`, return _a string array_ `answer` _(**1-indexed**) where_:

-   `answer[i] == "FizzBuzz"` if `i` is divisible by `3` and `5`.
-   `answer[i] == "Fizz"` if `i` is divisible by `3`.
-   `answer[i] == "Buzz"` if `i` is divisible by `5`.
-   `answer[i] == i` (as a string) if none of the above conditions are true.

### Example 1:

```
Input: n = 3
Output: ["1","2","Fizz"]
```

### Example 2:

```
Input: n = 5
Output: ["1","2","Fizz","4","Buzz"]
```

### Example 3:

```
Input: n = 15
Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]
```

### Constraints:

-   `1 <= n <= 10⁴`

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary><b>Reveal</b></summary>
  There are two approaches I will cover here, both of them should be very similar in terms of performance and memory usage (they both have the same time and space complexity).
  <p>
    
  <details>
  <summary>First approach</summary>

This approach is the more classic one in my opinion. You create an empty string, then if the number is divisible by 3 you append "Fizz" to the string, and if the number is divisible by 5 you append "Buzz" (in this order, since it gives you "FizzBuzz" when the number is divisible by 15). Then if the string is not empty, you push it to the array, else you push the string version of the number (because the number can't be divisible by 3 or 5, because if it was, it wouldn't be empty). _This approach is named "classic" in the table below._

  </details>
    
  <details>
  <summary>Second approach</summary>

This approach makes use of the match statement, where you match the results of taking the remainder of the number divided by 3 and by 5. If both of the results are 0 (the number is divisible by both 3 and 5) you append "FizzBuzz" to the answer array, else if the first result is 0 (the number is divisible by 3, but not 5) you append "Fizz", else if the second result is 0 (the number is divisible by 5, but not 3) you append "Buzz" and if you pass all of this, you append the string version of the number, since the number is not divisible by neither 3 nor 5. _This approach is named "matching" in the table below._

  </details>

  <p>
    
  |          Implementation         | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                     |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :--------------------------------------------------: |
  |     [Rust (classic)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0412/Rust/solution_classic.rs)     |       O(N)      |       O(N)       | 0 ms, faster than 100.00% of Rust online submissions | 2.6 MB, less than 80.67% of Rust online submissions |
  |       [Rust (matching)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0412/Rust/solution_matching.rs)      |       O(N)      |       O(N)       | 1 ms, faster than 72.67% of Rust online submissions |  2.6 MB, less than 80.67% of Rust online submissions |
</details>

## References

[https://leetcode.com/problems/fizz-buzz/](https://leetcode.com/problems/fizz-buzz/)
