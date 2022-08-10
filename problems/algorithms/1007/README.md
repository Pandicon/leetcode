# Problem #1007 - Minimum Domino Rotations For Equal Row

## Problem description

In a row of dominoes, `tops[i]` and `bottoms[i]` represent the top and bottom halves of the `iᵗʰ` domino. (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)

We may rotate the `iᵗʰ` domino, so that `tops[i]` and `bottoms[i]` swap values.

Return the minimum number of rotations so that all the values in `tops` are the same, or all the values in `bottoms` are the same.

If it cannot be done, return `-1`.

### Example 1:

(Also see the [image](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/1007/domino.png))

```
Input: tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
Output: 2
Explanation:
The first figure represents the dominoes as given by tops and bottoms: before we do any rotations.
If we rotate the second and fourth dominoes, we can make every value in the top row equal to 2, as indicated by the second figure.
```

### Example 2:

```
Input: tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
Output: -1
Explanation:
In this case, it is not possible to rotate the dominoes to make one row of values equal.
```

### Constraints:

-   `2 <= tops.length <= 2 * 10⁴`
-   `bottoms.length == tops.length`
-   `1 <= tops[i], bottoms[i] <= 6`

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary>Reveal</summary>
  
  We should realise that it is only possible to rotate the dominoes in the required way if all of them share at least one number. That means we only have to check the numbers that are on the first piece, since if a number isn't on it, a valid solution with it can't exist. Then we can loop through all of the dominoes for each of the numbers and keep the number of needed rotations for bottom and top row. We start it with either `n`, where `n` is the amount of the pieces. Then if we encounter the currently checked number in one of the rows, we subtract one from the corresponding counter. We also check if it is in at least one of the rows. If it is not, we mark it as invalid and break (no need to continue). If we check all dominoes and the number is still valid, we choose the minimum of the current minimum count, rotations needed for the top row, and rotations needed for the bottom row as the current minimum count. If we check both of the numbers that were on the first piece and at least one of them remained valid, we return the current minimum count, else we return `-1` (no solution was found).

  <p>
    
  |          Implementation         | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                     |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :--------------------------------------------------: |
  |     [Rust](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/1007/solution.rs)     |       O(N)      |       O(1)       | 18 ms, faster than 100.00% of Rust online submissions | 2.3 MB, less than 100.00% of Rust online submissions |
  |       [C](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/1007/solution.c)      |       O(N)      |       O(1)       | 183 ms, faster than 72.73% of C online submissions |  11.2 MB, less than 90.91% of C online submissions |
  |       [C++](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/1007/solution.cpp)      |       O(N)      |       O(1)       | 143 ms, faster than 95.36% of C++ online submissions |  111.5 MB, less than 85.68% of C++ online submissions |
</details>

## References

[https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/](https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/)
