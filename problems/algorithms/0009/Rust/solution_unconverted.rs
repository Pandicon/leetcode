impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x < 10 {
            return true;
        }
        let mut v = Vec::new();
        let mut num = x;
        while num > 0 {
            let units = num % 10;
            num /= 10;
            v.push(units);
        }
        let mut i = 0;
        let mut j = v.len() - 1;
        while i <= j {
            if v[i] != v[j] {
                return false;
            }
            i += 1;
            if j > 1 {
                j -= 1;                
            }
        }
        true
    }
}