class Solution
{
public:
	double myPow(double x, int n)
	{
		if (n == 0.0)
			return 1.0;
		if (n == 1.0 || x == 1.0)
			return x;
		double pow = 1.0;
		long abs_exponent = (long)n;
		if (abs_exponent < 0)
			abs_exponent *= -1;
		while (abs_exponent > 0)
		{
			if (abs_exponent & 1 == 1)
			{
				pow *= x;
			}
			x *= x;
			abs_exponent /= 2;
		}
		if (n < 0)
			return 1.0 / pow;
		return pow;
	}
};