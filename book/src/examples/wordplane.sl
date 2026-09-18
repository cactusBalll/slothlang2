// 词面编码：每个 SSA 值/槽/字段都是带 tag 的单 i64（ch05）
// 展开发射的 MLIR 可见：int 走 shl/shrsi、float 走 f64 解码、bool 走 0/2
func main() {
    print(1 + 2);        // int  算术在解码后的原始值上进行
    print(1.5 + 0.25);   // float
    print(3 < 4);        // bool
    print(nil is nil);   // nil 字面量 == 0
}
