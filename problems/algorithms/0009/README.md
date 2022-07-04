# Problem #9 - Palindrome Number (Easy)

## Problem description

Given an integer `x`, return `true` if `x` is palindrome integer.
An integer is a **palindrome** when it reads the same backward as forward.

-   For example, `121` is a palindrome while `123` is not.

**Follow up:** Could you solve it without converting the integer to a string?

### Example 1:

```
Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
```

### Example 2:

```
Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
```

### Example 3:

```
Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
```

### Constraints:

-   `-2³¹ <= x <= 2³¹ - 1`

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary>Reveal</summary>
  There are two approaches I will cover here.
  <p>
    
**First approach:** The more obvious approach is to convert the number to a string and then read the string from the start and from the end at the same time, checking if you are encountering the same characters. If not, it is not a palindrome, so you return false, else you return true. You can also return false whenever you get a negative number, because of the negation symbol. After that, you can also return true for all numbers lower than 10, since they are single digit and therefore have to read the same from the front and from the back. _This approach is named "converted" in the table below)_

**Second approach:** This approach doesn't convert the number to a string, but rather works with the number itself to create an array of digits it consists of. First you return false whenever you get a negative number, because of the negation symbol. After that, you also return true for all numbers lower than 10, since they are single digit and therefore have to read the same from the front and from the back. Then you loop until the number is lower than 1 and split off the units by taking the current number modulo 10. You push this to the vector of digits, and then divide the current number by 10 (using integer division, this will leave you with the rest of the number). Once you get all of the digits, you can just loop through them both from the start and from the end and check if you are encountering the same digits. Once you encounter a pair of different digits, you return false. If you don't encounter such a pair, you can safely return true. _This approach is named "unconverted" in the table below)_

  <p>
    
  |          Implementation         | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                     |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :--------------------------------------------------: |
  |     [Rust (converted)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0009/Rust/solution_converted.rs)     |       O(N)      |       O(N)       | 12 ms, faster than 64.35% of Rust online submissions | 2.3 MB, less than 15.35% of Rust online submissions |
  |     [Rust (unconverted)](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0009/Rust/solution_unconverted.rs)     |       O(N)      |       O(N)       | 3 ms, faster than 97.58% of Rust online submissions | 2.1 MB, less than 68.14% of Rust online submissions |
</details>

## References

[https://leetcode.com/problems/palindrome-number/](https://leetcode.com/problems/palindrome-number/)
