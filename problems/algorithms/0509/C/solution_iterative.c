int fib(int n)
{
	if (n < 2)
		return n;
	int res = 1;
	int prev = 0;
	for (int i = 2; i <= n; i++)
	{
		res += prev;
		prev = res - prev;
	}
	return res;
}