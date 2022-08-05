

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *rearrangeArray(int *nums, int numsSize, int *returnSize)
{
	int *positive_numbers = malloc(numsSize * sizeof(int) / 2);
	int *negative_numbers = malloc(numsSize * sizeof(int) / 2);
	int positive_index = 0;
	int negative_index = 0;
	int *result = malloc(numsSize * sizeof(int));
	for (int i = 0; i < numsSize; i += 1)
	{
		int num = nums[i];
		if (num > 0)
			positive_numbers[positive_index++] = num;
		else
			negative_numbers[negative_index++] = num;
	}
	for (int i = 0; i < numsSize / 2; i += 1)
	{
		result[i * 2] = positive_numbers[i];
		result[i * 2 + 1] = negative_numbers[i];
	}
	*returnSize = numsSize;
	return result;
}