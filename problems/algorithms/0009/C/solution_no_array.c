bool isPalindrome(int x)
{
	if (x < 0)
		return false;
	if (x < 10)
		return true;
	long reversed = 0;
	int units = 0;
	int temporary = x;
	while (temporary != 0)
	{
		units = temporary % 10;
		reversed = reversed * 10 + units;
		temporary /= 10;
	}
	return x == reversed;
}