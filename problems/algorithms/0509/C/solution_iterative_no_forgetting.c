int calculated[31] = {0};

int fib(int n)
{
	if (n < 2)
		return n;
	calculated[1] = 1;
	for (int i = 2; i <= n; i++)
	{
		calculated[i] = calculated[i - 1] + calculated[i - 2];
	}
	return calculated[n];
}