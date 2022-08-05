

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *rearrangeArray(int *nums, int numsSize, int *returnSize)
{
	int *result = malloc(numsSize * sizeof(int));
	int positive_index = 0;
	int negative_index = 1;
	for (int i = 0; i < numsSize; i += 1)
	{
		int num = nums[i];
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
	*returnSize = numsSize;
	return result;
}