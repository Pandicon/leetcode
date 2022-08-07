

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *pivotArray(int *nums, int numsSize, int pivot, int *returnSize)
{
	int *result = (int *)malloc(numsSize * sizeof(int));
	*returnSize = 0;
	if (result == NULL)
		return NULL;
	int equal_count = 0;
	int higher[100000];
	int higher_count = 0;
	for (int i = 0; i < numsSize; i += 1)
	{
		if (nums[i] == pivot)
			equal_count += 1;
		else if (nums[i] < pivot)
			result[(*returnSize)++] = nums[i];
		else
		{
			higher[higher_count++] = nums[i];
		}
	}
	for (int i = 0; i < equal_count; i += 1)
	{
		result[(*returnSize)++] = pivot;
	}
	for (int i = 0; i < higher_count; i += 1)
	{
		result[(*returnSize)++] = higher[i];
	}
	return result;
}