impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut equal = 0;
        let mut higher = vec![];
        for &num in &nums {
            if num == pivot {
                equal += 1;
            } else if num < pivot {
                result.push(num);
            } else {
                higher.push(num);
            }
        }
        for _ in 0..equal {
            result.push(pivot);
        }
        result.append(&mut higher);
        result
    }
}