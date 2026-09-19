// 词面编码：每个 SSA 值/槽/字段都是单个 i64（无 tag，ch05）
// 展开发射的 MLIR 可见：int 直通、float 走 llvm.bitcast、bool 走 0/1
func main() {
    print(1 + 2);        // int  算术就在原生 i64 上
    print(1.5 + 0.25);   // float
    print(3 < 4);        // bool
    print(nil is nil);   // nil 字面量 == 0
}
