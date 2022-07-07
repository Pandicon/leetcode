impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let c = n as usize;
        let mut calculated = vec![0; c + 1];
        calculated[1] = 1;
        for i in 2..=c {
            calculated[i] = calculated[i - 1] + calculated[i - 2];
        }
        calculated[c]
    }
}