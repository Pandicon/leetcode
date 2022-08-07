

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *pivotArray(int *nums, int numsSize, int pivot, int *returnSize)
{
	int *result = (int *)malloc(numsSize * sizeof(int));
	if (result == NULL)
		return NULL;
	int lower_count = 0;
	int equal_count = 0;
	for (int i = 0; i < numsSize; i += 1)
	{
		if (nums[i] == pivot)
			equal_count += 1;
		else if (nums[i] < pivot)
			lower_count += 1;
	}
	int lower_index = 0;
	int equal_index = lower_count;
	int higher_index = lower_count + equal_count;
	for (int i = 0; i < numsSize; i += 1)
	{
		if (nums[i] == pivot)
			result[equal_index++] = nums[i];
		else if (nums[i] < pivot)
			result[lower_index++] = nums[i];
		else
			result[higher_index++] = nums[i];
	}
	*returnSize = numsSize;
	return result;
}