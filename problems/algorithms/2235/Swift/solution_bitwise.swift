class Solution {
    func sum(_ num1: Int, _ num2: Int) -> Int {
        var a: Int = num1;
        var b: Int = num2;
        while(b != 0) {
            let carry: Int = a & b;
            a = a ^ b;
            b = carry << 1;
        }
        return a;
    }
}