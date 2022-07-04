impl Solution {	
	pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut positive_index = 0;
        let mut negative_index = 1;
        let mut res = vec![0; nums.len()];
        for num in nums {
            if num > 0 {
                res[positive_index] = num;
                positive_index += 2;
            } else {
                res[negative_index] = num;
                negative_index += 2;
            }
        }
        res
    }
}