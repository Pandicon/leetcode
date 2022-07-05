impl Solution {
	pub fn fizz_buzz(n: i32) -> Vec<String> {
		let mut answer = Vec::new();
		for i in 1..=n {
			let mut to_push = String::with_capacity(8);
			if i % 3 == 0 {
				to_push += "Fizz";
			}
			if i % 5 == 0 {
				to_push += "Buzz";
			}
			if to_push.is_empty() {
				to_push = i.to_string();
			}
			answer.push(to_push);
		}
		answer
	}
}