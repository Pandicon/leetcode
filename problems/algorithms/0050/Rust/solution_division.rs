impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let mut pow = 1.0;
        let mut base = x;
        let mut abs_exp = n.abs() as u32;
        while abs_exp > 0 {
            if abs_exp & 1 == 1 {
                pow *= base;
            }
            base *= base;
            abs_exp /= 2;
        }
        if n < 0 {
            return 1.0 / pow;
        }
        pow
    }
}