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
		unsigned int num = 1 << 31;
		for (int i = 0; i < 32; i++)
		{
			int is_bit_one = ((abs_exponent & num) != 0);
			pow *= pow;
			if (is_bit_one)
			{
				pow *= x;
			}
			num = num >> 1;
		}
		if (n < 0)
			return 1.0 / pow;
		return pow;
	}
};