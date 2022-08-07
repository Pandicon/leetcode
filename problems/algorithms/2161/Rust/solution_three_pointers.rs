impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut lower = 0;
        let mut equal = 0;
        for &num in &nums {
            if num == pivot {
                equal += 1;
            } else if num < pivot {
                lower += 1;
            }
        }
        let mut lower_index = 0;
        let mut equal_index = lower;
        let mut higher_index = lower + equal;
        for &num in &nums {
            if num == pivot {
                result[equal_index] = num;
                equal_index += 1;
            } else if num < pivot {
                result[lower_index] = num;
                lower_index += 1;
            } else {
                result[higher_index] = num;
                higher_index += 1;
            }
        }
        result
    }
}