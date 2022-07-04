impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index_num = Vec::new();
        for (i, num) in nums.iter().enumerate() {
            index_num.push([i as i32, *num]);
        }
        index_num.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());
        let mut i = 0;
        let mut j = index_num.len() - 1;
        while i < j {
            let num1 = index_num[i][1];
            let num2 = index_num[j][1];
            let sum = num1 + num2;
            if sum == target {
                return [index_num[i][0], index_num[j][0]].to_vec();
            } else if sum > target {
                j -= 1;
            } else if sum < target {
                i += 1;
            }
        }
        [0, 0].to_vec()
    }
}