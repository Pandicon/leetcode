#include <math.h>

double PHI = 1.61803398875;
double SQRT_5 = 2.2360679775;

int fib(int n)
{
	if (n < 2)
		return n;
	return round((pow(PHI, n) - pow(1 - PHI, n)) / SQRT_5);
}