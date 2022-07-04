impl Solution {
	pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut positive_nums = Vec::new();
        let mut negative_nums = Vec::new();
        let mut res = Vec::new();
        for num in nums {
            if num > 0 {
                positive_nums.push(num);
            } else {
                negative_nums.push(num);
            }
        }
        for i in 0..positive_nums.len() {
            res.push(positive_nums[i]);
            res.push(negative_nums[i]);
        }
        res
    }
	
	pub fn rearrange_array_two_pointers(nums: Vec<i32>) -> Vec<i32> {
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