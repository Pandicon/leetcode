impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let i = n as usize;
        let mut already_calculated = vec![0; i + 1];
        Solution::fib_helper(i - 1, &mut already_calculated) + Solution::fib_helper(i - 2, &mut already_calculated)
    }
    
    fn fib_helper(n: usize, calculated: &mut Vec<i32>) -> i32 {
        if n < 2 {
            return n as i32;
        }
        if calculated[n] != 0 {
            return calculated[n];
        }
        calculated[n] = Solution::fib_helper(n - 1, calculated) + Solution::fib_helper(n - 2, calculated);
        calculated[n]
    }
}