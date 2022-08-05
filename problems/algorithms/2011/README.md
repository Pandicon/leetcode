# Problem #2011 - Final Value of Variable After Performing Operations (Easy)

## Problem description

There is a programming language with only **four** operations and **one** variable X:

-   `++X` and `X++` **increments** the value of the variable `X` by `1`.
-   `--X` and `X--` **decrements** the value of the variable `X` by `1`.

Initially, the value of `X` is `0`.
Given an array of strings `operations` containing a list of operations, return _the **final** value of `X` after performing all the operations_.

### Example 1:

```
Input: operations = ["--X","X++","X++"]
Output: 1
Explanation: The operations are performed as follows:
Initially, X = 0.
--X: X is decremented by 1, X =  0 - 1 = -1.
X++: X is incremented by 1, X = -1 + 1 =  0.
X++: X is incremented by 1, X =  0 + 1 =  1.
```

### Example 2:

```
Input: operations = ["++X","++X","X++"]
Output: 3
Explanation: The operations are performed as follows:
Initially, X = 0.
++X: X is incremented by 1, X = 0 + 1 = 1.
++X: X is incremented by 1, X = 1 + 1 = 2.
X++: X is incremented by 1, X = 2 + 1 = 3.
```

### Example 3:

```
Input: operations = ["X++","++X","--X","X--"]
Output: 0
Explanation: The operations are performed as follows:
Initially, X = 0.
X++: X is incremented by 1, X = 0 + 1 = 1.
++X: X is incremented by 1, X = 1 + 1 = 2.
--X: X is decremented by 1, X = 2 - 1 = 1.
X--: X is decremented by 1, X = 1 - 1 = 0.
```

### Constraints:

-   `1 <= operations.length <= 100`
-   `operations[i]` will be either `"++X"`, `"X++"`, `"--X"`, or `"X--"`.

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary>Reveal</summary>

This problem is quite simple, as there are only four possible values that we could get as each element of the input array, so we could check for all of them:
`if element = X++ or element = ++X, increment, else decrement`
However, when you take a look at the possible values, you can see a pattern:

```
X + +
+ + X
  ^

X - -
- - X
  ^
```

Both of the increment and both of the decrement commands have `+` and `-` in the 2nd spot. That means you can only check the 2nd spot of the command.

  <p>
    
  |          Implementation         | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                     |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :--------------------------------------------------: |
  |     [Rust](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2011/solution.rs)     |       O(N)      |       O(1)       | 0 ms, faster than 100.00% of Rust online submissions | 2.1 MB, less than 87.93% of Rust online submissions |
  |       [C](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2011/solution.c)      |       O(N)      |       O(1)       | 5 ms, faster than 83.91% of C online submissions |  6.6 MB, less than 42.74% of C online submissions |
  |       [C++](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/2011/solution.cpp)      |       O(N)      |       O(1)       | 11 ms, faster than 71.60% of C++ online submissions |  13.9 MB, less than 92.88% of C++ online submissions |
</details>

## References

[https://leetcode.com/problems/final-value-of-variable-after-performing-operations/](https://leetcode.com/problems/final-value-of-variable-after-performing-operations/)
