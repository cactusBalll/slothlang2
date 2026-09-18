func main() {
    print(42);                 // print(int)
    print(2.5);                // print(float)
    print(true);               // print(bool) -> true/false
    print("s");                // print(str)

    print(len("abc"));         // len(str) 字节数
    print(len([1, 2, 3]));     // len(Array)
    print(len(@(1: 1)));       // len(Map)

    print(int(3.9));           // 3   显式截断转换
    print(float(3));           // 3   int -> float

    let m = @("k": 1);
    print(keys(m).len());      // 1
    print(values(m).len());    // 1

    // rc 诊断面（SLOTH_STATS 内置）
    print(sloth_rc_live() >= 0);// true
}
