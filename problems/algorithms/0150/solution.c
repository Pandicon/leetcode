#include <stdio.h>

int evalRPN(char **tokens, int tokensSize)
{
	int *stack = malloc(tokensSize * sizeof(int));
	if (stack == NULL)
	{
		printf("Failed to allocate array");
		return 1;
	}
	int stack_length = 0;
	int token_index = 0;
	while (token_index < tokensSize)
	{
		if (strcmp(tokens[token_index], "+") == 0)
		{
			stack[stack_length - 2] += stack[stack_length - 1];
			stack[stack_length - 1] = 0;
			stack_length--;
		}
		else if (strcmp(tokens[token_index], "-") == 0)
		{
			stack[stack_length - 2] -= stack[stack_length - 1];
			stack[stack_length - 1] = 0;
			stack_length--;
		}
		else if (strcmp(tokens[token_index], "*") == 0)
		{
			stack[stack_length - 2] *= stack[stack_length - 1];
			stack[stack_length - 1] = 0;
			stack_length--;
		}
		else if (strcmp(tokens[token_index], "/") == 0)
		{
			stack[stack_length - 2] /= stack[stack_length - 1];
			stack[stack_length - 1] = 0;
			stack_length--;
		}
		else
		{
			stack[stack_length] = atoi(tokens[token_index]);
			stack_length++;
		}
		token_index++;
	}
	return stack[0];
}