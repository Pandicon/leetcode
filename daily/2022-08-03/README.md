# 03/08/2022 - My Calendar I (Medium), Problem #729

The daily folder only includes the solutions and implementations I came up while the problem was a daily problem. If you want to see the other ones for this problem, [check it out in the problems directory](https://github.com/Pandicon/leetcode/tree/main/problems/algorithms/0729).

## Problem description

You are implementing a program to use as your calendar. We can add a new event if adding the event will not cause a **double booking**.
A **double booking** happens when two events have some non-empty intersection (i.e., some moment is common to both events.).
The event can be represented as a pair of integers `start` and `end` that represents a booking on the half-open interval `[start, end)`, the range of real numbers `x` such that `start <= x < end`.
Implement the `MyCalendar` class:

-   `MyCalendar()` Initializes the calendar object.
-   `boolean book(int start, int end)` Returns `true` if the event can be added to the calendar successfully without causing a **double booking**. Otherwise, return `false` and do not add the event to the calendar.

### Example 1:

```
Input
["MyCalendar", "book", "book", "book"]
[[], [10, 20], [15, 25], [20, 30]]
Output
[null, true, false, true]

Explanation
MyCalendar myCalendar = new MyCalendar();
myCalendar.book(10, 20); // return True
myCalendar.book(15, 25); // return False, It can not be booked because time 15 is already booked by another event.
myCalendar.book(20, 30); // return True, The event can be booked, as the first event takes every time less than 20, but not including 20.
```

### Constraints:

-   `0 <= start < end <= 10⁹`
-   At most `1000` calls will be made to `book`.

## My approach

**Warning: Try to solve the problem on your own before reading this, since it will spoil the solution.**

<details>
  <summary>Reveal</summary>

To be able to book an event, its interval must not overlap with the interval of any other event. Since we will not be changing starting times of events, we can use a BTreeMap to store the events sorted in ascending order by their start time.
An event is not overlapping if:

-   It starts after the previous one ends
-   It ends before the next one starts

We can use the `.range()` method of the BTreeMap to get all events that start before the new event ends. This way we don't have to manually check if the event ends before the next one starts, because the method will filter it for us.
Because we know that the map will only contain non-overlapping events, we can only check the last returned event. If that event ends after the new event starts, it is overlapping with it.

  <p>
    
  |         Implementation          | Time complexity | Space complexity |                        Runtime                       |                     Memory Usage                    |
  | :-----------------------------: | :-------------: | :--------------: | :--------------------------------------------------: | :-------------------------------------------------: |
  |              [Rust](https://github.com/Pandicon/leetcode/tree/main/daily/2022-08-03/solution.rs)               |       O(N*log(N))      |       O(N)       | 33 ms, faster than 83.33% of Rust online submissions | 2.7 MB, less than 88.89% of Rust online submissions |
</details>

## References

[https://leetcode.com/problems/my-calendar-i/](https://leetcode.com/problems/my-calendar-i/)
