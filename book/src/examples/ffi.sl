// FFI：extern func 走 C ABI；extern type 为不透明类型
extern func sloth_extern_floor(x: float): float;
extern func sloth_extern_powf(a: float, b: float): float;

extern type Tok;                                  // 不透明句柄
extern func sloth_extern_tok_new(): Tok;
extern func sloth_extern_tok_val(t: Tok): int;

func main() {
    print(sloth_extern_floor(2.7));               // 2
    print(sloth_extern_powf(2.0, 10.0));          // 1024
    let t = sloth_extern_tok_new();
    print(sloth_extern_tok_val(t));               // 99
}
