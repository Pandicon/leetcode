class Solution
{
public:
	int finalValueAfterOperations(vector<string> &operations)
	{
		int val = 0;
		for (string operation : operations)
		{
			if (operation[1] == '+')
				val++;
			else
				val--;
		}
		return val;
	}
};