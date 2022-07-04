impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x == 0 {
            return true;
        }
        let string = x.to_string();
        let split = string.split("");
        let str_vec: Vec<&str> = split.collect();
        let mut i = 1;
        let mut j = str_vec.len() - 2;
        while i <= j {
            if str_vec[i] != str_vec[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}