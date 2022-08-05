impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut val = 0;
        for operation in operations.iter() {
            if operation.chars().nth(1).unwrap() == '+' {
                val += 1;
            } else {
                val -= 1;
            }
        }
        val
    }
}