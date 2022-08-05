class Solution
{
public:
	vector<int> rearrangeArray(vector<int> &nums)
	{
		int positive_numbers_count = nums.size() / 2;
		vector<int> positive_numbers, negative_numbers, result;
		for (int num : nums)
		{
			if (num > 0)
				positive_numbers.push_back(num);
			else
				negative_numbers.push_back(num);
		}
		for (int i = 0; i < positive_numbers_count; i += 1)
		{
			result.push_back(positive_numbers[i]);
			result.push_back(negative_numbers[i]);
		}
		return result;
	}
};