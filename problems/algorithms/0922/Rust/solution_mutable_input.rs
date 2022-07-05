impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();
        let mut stack: [Vec<(usize, i32)>; 2] = [Vec::new(), Vec::new()];
        for i in 0..nums_len {
            let i_parity = i % 2;
            let num = nums[i];
            if i_parity != num as usize % 2 {
                if let Some(pair) = stack[i_parity].pop() {
                    nums[pair.0] = num;
                    nums[i] = pair.1;
                } else {
                    stack[(i_parity + 1) % 2].push((i, num));
                }
            } else {
                nums[i] = num;
            }
        }
        nums
    }
}