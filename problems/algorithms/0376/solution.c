int wiggleMaxLength(int *nums, int numsSize)
{
	if (numsSize == 1)
		return 1;
	if (numsSize == 2)
	{
		if (nums[0] == nums[1])
			return 1;
		return 2;
	}
	int dir = 0;
	int max_len = 1;
	for (int i = 1; i < numsSize; i++)
	{
		int diff = nums[i] - nums[i - 1];
		if (diff == 0)
			continue;
		int diff_sign = ((diff > 0) - (diff < 0));
		if (dir == 0 || dir != diff_sign)
		{
			dir = diff_sign;
			max_len++;
		}
	}
	return max_len;
}