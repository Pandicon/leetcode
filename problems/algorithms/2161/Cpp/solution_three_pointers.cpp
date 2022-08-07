class Solution
{
public:
	vector<int> pivotArray(vector<int> &nums, int pivot)
	{
		vector<int> result(nums.size());
		int lower = 0;
		int equal = 0;
		for (int num : nums)
		{
			if (num == pivot)
			{
				equal += 1;
			}
			else if (num < pivot)
			{
				lower += 1;
			}
		}
		int lower_index = 0;
		int equal_index = lower;
		int higher_index = lower + equal;
		for (int num : nums)
		{
			if (num == pivot)
			{
				result[equal_index++] = num;
			}
			else if (num < pivot)
			{
				result[lower_index++] = num;
			}
			else
			{
				result[higher_index++] = num;
			}
		}
		return result;
	}
};