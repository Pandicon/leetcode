int calculated[31] = {0};

int fib(int n)
{
	if (n < 2)
		return n;
	if (calculated[n] != 0)
		return calculated[n];
	return calculated[n] = fib(n - 1) + fib(n - 2);
}