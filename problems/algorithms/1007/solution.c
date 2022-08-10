int min(int, int);

int minDominoRotations(int *tops, int topsSize, int *bottoms, int bottomsSize)
{
	int to_check[2] = {tops[0], bottoms[0]};
	int is_valid[2] = {1, 1};
	int current_min = topsSize + 1;
	for (int i = 0; i < 2; i += 1)
	{
		int checking = to_check[i];
		int top_switches = topsSize;
		int bottom_switches = bottomsSize;
		for (int j = 0; j < topsSize; j += 1)
		{
			int is_top_match = checking == tops[j];
			int is_bottom_match = checking == bottoms[j];
			if (!is_top_match && !is_bottom_match)
			{
				is_valid[i] = 0;
				break;
			}
			top_switches -= is_top_match;
			bottom_switches -= is_bottom_match;
		}
		if (is_valid[i])
		{
			current_min = min(min(top_switches, bottom_switches), current_min);
		}
	}
	if (is_valid[0] || is_valid[1])
		return current_min;
	return -1;
}

int min(int x, int y)
{
	if (x > y)
		return y;
	return x;
}