impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut table = std::collections::HashMap::<i32, usize>::new();
        for i in 0..nums.len() {
            let current_num = nums[i];
            let difference = target - current_num;
            if let Some(index) = table.get(&difference) {
                return vec![*index as i32, i as i32];
            } else {
                table.insert(current_num, i);
            }
        }
        vec![0, 0]
    }
}