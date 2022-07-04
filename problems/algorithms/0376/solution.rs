impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        if nums.len() == 2 {
            if nums[0] == nums[1] {
                return 1;
            } else {
                return 2;
            }
        }
        let mut dir = 0;
        let mut max_len = 1;
        for i in 1..nums.len() {
            let diff =  nums[i] - nums[i - 1];
            if diff == 0 {
                continue;
            }
            let diff_sign = diff/diff.abs();
            if dir == 0 || diff_sign != dir {
                dir = diff_sign;
                max_len += 1;
            }
        }
        max_len
    }
}