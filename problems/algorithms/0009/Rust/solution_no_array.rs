impl Solution {
	pub fn is_palindrome(x: i32) -> bool {
		if x < 0 {
			return false;
		}
		if x < 10 {
			return true;
		}
		let mut reversed = 0;
		let mut temporary = x;
		while temporary != 0 {
			let units = temporary % 10;
			reversed = reversed * 10 + units;
			temporary /= 10;
		}
		x == reversed
	}
}