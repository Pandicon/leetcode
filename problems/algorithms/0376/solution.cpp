class Solution
{
public:
	int wiggleMaxLength(vector<int> &nums)
	{
		int nums_size = nums.size();
		if (nums_size == 1)
			return 1;
		if (nums_size == 2)
		{
			if (nums[0] == nums[1])
				return 1;
			else
				return 2;
		}
		int dir = 0;
		int max_length = 1;
		for (int i = 1; i < nums_size; i++)
		{
			int diff = nums[i] - nums[i - 1];
			if (diff == 0)
				continue;
			int diff_sign = ((diff > 0) - (diff < 0));
			if (dir == 0 || diff_sign != dir)
			{
				dir = diff_sign;
				max_length += 1;
			}
		}
		return max_length;
	}
};