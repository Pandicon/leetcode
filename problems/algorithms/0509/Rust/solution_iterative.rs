impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let mut res = 1;
        let mut prev = 0;
        for _ in 2..=n {
            res += prev;
            prev = res - prev;
        }
        res
    }
}