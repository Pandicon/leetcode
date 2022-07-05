impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut stack: [Vec<(usize, i32)>; 2] = [Vec::new(), Vec::new()];
        for (i, &num) in nums.iter().enumerate() {
            let i_parity = i % 2;
            if i_parity != num as usize % 2 {
                if let Some(pair) = stack[i_parity].pop() {
                    res[pair.0] = num;
                    res[i] = pair.1;
                } else {
                    stack[(i_parity + 1) % 2].push((i, num));
                }
            } else {
                res[i] = num;
            }
        }
        res
    }
}