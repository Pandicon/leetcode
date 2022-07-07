const PHI: f64 = 1.61803398875;
const SQRT_5: f64 = 2.2360679775;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        ((PHI.powi(n) - (1.0 - PHI).powi(n))/SQRT_5).round() as i32
    }
}