class Solution
{
public:
	int minDominoRotations(vector<int> &tops, vector<int> &bottoms)
	{
		int n = tops.size();
		int to_check[2] = {tops[0], bottoms[0]};
		bool is_valid[2] = {true, true};
		int current_min = n + 1;
		for (int i = 0; i < 2; i += 1)
		{
			int checking = to_check[i];
			int top_switches = n;
			int bottom_switches = n;
			for (int j = 0; j < n; j += 1)
			{
				int is_top_match = checking == tops[j];
				int is_bottom_match = checking == bottoms[j];
				if (!is_top_match && !is_bottom_match)
				{
					is_valid[i] = 0;
					break;
				}
				top_switches -= is_top_match;
				bottom_switches -= is_bottom_match;
			}
			if (is_valid[i])
			{
				current_min = min({current_min, top_switches, bottom_switches});
			}
		}
		if (is_valid[0] || is_valid[1])
			return current_min;
		return -1;
	}
};