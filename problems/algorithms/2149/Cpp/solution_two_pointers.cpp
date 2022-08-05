class Solution
{
public:
	vector<int> rearrangeArray(vector<int> &nums)
	{
		int n = nums.size();
		vector<int> result(n);
		int positive_index = 0;
		int negative_index = 1;
		for (int num : nums)
		{
			if (num > 0)
			{
				result[positive_index] = num;
				positive_index += 2;
			}
			else
			{
				result[negative_index] = num;
				negative_index += 2;
			}
		}
		return result;
	}
};