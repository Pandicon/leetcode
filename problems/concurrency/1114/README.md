# Problem #1114 - Print in Order (Easy)

## Problem description

Suppose we have a class:

```
public class Foo {
  public void first() { print("first"); }
  public void second() { print("second"); }
  public void third() { print("third"); }
}
```

The same instance of `Foo` will be passed to three different threads. Thread A will call `first()`, thread B will call `second()`, and thread C will call `third()`. Design a mechanism and modify the program to ensure that `second()` is executed after `first()`, and `third()` is executed after `second()`.

**Note:** We do not know how the threads will be scheduled in the operating system, even though the numbers in the input seem to imply the ordering. The input format you see is mainly to ensure our tests' comprehensiveness.

### Example 1:

```
Input: nums = [1,2,3]
Output: "firstsecondthird"
Explanation: There are three threads being fired asynchronously. The input [1,2,3] means thread A calls first(), thread B calls second(), and thread C calls third(). "firstsecondthird" is the correct output.
```

### Example 2:

```
Input: nums = [1,3,2]
Output: "firstsecondthird"
Explanation: The input [1,3,2] means thread A calls first(), thread B calls third(), and thread C calls second(). "firstsecondthird" is the correct output.
```

### Constraints:

-   `nums` is a permutation of `[1, 2, 3]`

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary>Reveal</summary>
  There are two approaches I will cover here, one of them is very brainless, the other one is more correct.
  <p>
    
**First approach:** This approach is quite simple. You creare two booleans, indicating if the first and second functions ran and initialise them to false. Then once the function corresponding to the variable runs, you change the value to true. You then make the second function wait until the boolean indicating the first function ran is true, same for the third function and second boolean. _This approach is named as "brainless" in the table below._

**Second approach:** You can also make use of locks by acquiring them at initialisation, and releasing them once the function runs. Then you only let the next function run once the lock is released. _This approach is named as "locks" in the table below._

  <p>
    
  |          Implementation         | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                     |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :--------------------------------------------------: |
  |     [Python 3 (brainless)](https://github.com/Pandicon/leetcode/tree/main/problems/concurrency/1114/Python3/solution_brainless.py)     |       O(1)      |       O(1)       | 2774 ms, faster than 21.64 % of Python3 online submissions | 14.5 MB, less than 56.05% of Python3 online submissions |
  |       [Python 3 (locks)](https://github.com/Pandicon/leetcode/tree/main/problems/concurrency/1114/Python3/solution_locks.py)      |       O(1)      |       O(1)       | 49 ms, faster than 89.68% of Python3 online submissions |  14.4 MB, less than 56.05% of Python3 online submissions |
</details>

## References

[https://leetcode.com/problems/print-in-order](https://leetcode.com/problems/print-in-order)
