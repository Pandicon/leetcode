func sum(num1 int, num2 int) int {
    a := num1;
    b := num2;
    for b != 0 {
        carry := a & b;
        a = a ^ b;
        b = carry << 1;
    }
    return a;
}