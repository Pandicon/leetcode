impl Solution {
	pub fn fizz_buzz(n: i32) -> Vec<String> {
		let mut answer = Vec::new();
		for i in 1..=n {
			match (i % 3, i % 5) {
				(0, 0) => answer.push(String::from("FizzBuzz")),
				(0, _) => answer.push(String::from("Fizz")),
				(_, 0) => answer.push(String::from("Buzz")),
				(_, _) => answer.push(i.to_string()),
			}
		}
		answer
	}
}