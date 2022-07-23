# Problem #150 - Evaluate Reverse Polish Notation (Medium)

## Problem description

Evaluate the value of an arithmetic expression in [Reverse Polish Notation](http://en.wikipedia.org/wiki/Reverse_Polish_notation).

Valid operators are `+`, `-`, `*`, and `/`. Each operand may be an integer or another expression.

**Note** that division between two integers should truncate toward zero.

It is guaranteed that the given RPN expression is always valid. That means the expression would always evaluate to a result, and there will not be any division by zero operation.

**Example 1:**

<pre>Input: tokens = ["2","1","+","3","*"]
Output: 9
Explanation: ((2 + 1) * 3) = 9
</pre>

**Example 2:**

<pre>Input: tokens = ["4","13","5","/","+"]
Output: 6
Explanation: (4 + (13 / 5)) = 6
</pre>

**Example 3:**

<pre>Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
Output: 22
Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
= ((10 * (6 / (12 * -11))) + 17) + 5
= ((10 * (6 / -132)) + 17) + 5
= ((10 * 0) + 17) + 5
= (0 + 17) + 5
= 17 + 5
= 22
</pre>

**Constraints:**

-   `1 <= tokens.length <= 10⁴`
-   `tokens[i]` is either an operator: `"+"`, `"-"`, `"*"`, or `"/"`, or an integer in the range `[-200, 200]`.

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary><b>Reveal</b></summary>
  The notation always takes the last two values seen into account when an operator is present, including results of previous calculations. Due to this, you can store the numbers and calculated values in one stack, always popping the two last ones when you encounter an operator, use them, and push the result back to the stack.
  <p>
    
  |         Implementation          | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                    |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :-------------------------------------------------: |
  |              [Rust](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0150/solution.rs)               |       O(N)      |       O(N)       | 0 ms, faster than 100.00% of Rust online submissions | 2.7 MB, less than 86.84% of Rust online submissions |
  |              [C](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0150/solution.c)               |       O(N)      |       O(N)       | 6 ms, faster than 93.75% of C online submissions | 7.4 MB, less than 65.00% of C online submissions |
</details>

## References

[https://leetcode.com/problems/evaluate-reverse-polish-notation/](https://leetcode.com/problems/evaluate-reverse-polish-notation/)
