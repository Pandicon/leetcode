impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();
        let mut even_index: usize = 0;
        let mut odd_index: usize = 1;
        while even_index < nums_len && odd_index < nums_len {
            nums.swap(even_index, odd_index);
            while even_index < nums_len && nums[even_index] % 2 == 0 {
                even_index += 2;
            }
            while odd_index < nums_len && nums[odd_index] % 2 == 1 {
                odd_index += 2;
            }
        }
        nums
    }
}