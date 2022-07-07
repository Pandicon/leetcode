impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        Solution::fib(n - 1) + Solution::fib(n - 2)
    }
}