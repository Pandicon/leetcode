impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let mut pow = 1.0;
        let abs_exp = n.abs() as u32;
        let mut num: u32 = 1 << 31;
        for i in 0..32 {
            let is_bit_one = (abs_exp & num != 0);
            pow *= pow;
            if is_bit_one {
                pow *= x;
            }
            num = num >> 1;
        }
        if n < 0 {
            return 1.0 / pow;
        }
        pow
    }
}