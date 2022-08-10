impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let n = tops.len();
        let to_check = [tops[0], bottoms[0]];
        let mut is_valid = [true, true];
        let mut current_min = n + 1;
        for i in 0..2 {
            let checking = to_check[i];
            let mut top_switches = n;
            let mut bottom_switches = n;
            for j in 0..n {
                let is_top_match = checking == tops[j];
                let is_bottom_match = checking == bottoms[j];
                if (!is_top_match && !is_bottom_match) {
					is_valid[i] = false;
					break;
				}
                if is_top_match {
                    top_switches -= 1;
                }
                if is_bottom_match {
                    bottom_switches -= 1;
                }
            }
            if is_valid[i] {
                current_min = *[current_min, top_switches, bottom_switches].iter().min().unwrap();
            }
        }
        if is_valid[0] || is_valid[1] {
            return current_min as i32;
        }
        -1
    }
}