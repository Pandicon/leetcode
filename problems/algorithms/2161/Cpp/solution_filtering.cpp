class Solution
{
public:
	vector<int> pivotArray(vector<int> &nums, int pivot)
	{
		vector<int> result, higher;
		int equal = 0;
		for (int num : nums)
		{
			if (num == pivot)
			{
				equal += 1;
			}
			else if (num < pivot)
			{
				result.push_back(num);
			}
			else
			{
				higher.push_back(num);
			}
		}
		result.insert(std::end(result), equal, pivot);
		result.insert(std::end(result), std::begin(higher), std::end(higher));
		return result;
	}
};