impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while i < j {
            let num1 = numbers[i];
            let num2 = numbers[j];
            if num1 + num2 == target {
                return vec![i as i32 + 1, j as i32 + 1];
            } else if num1 + num2 < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        return vec![0, 0];
    }
}