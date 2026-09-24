# 9. 函数

## 9.1 声明

```sloth
func add(a: int, b: int): int {
    return a + b;
}
func greet(name: str): unit {   // unit 返回
    print("hi ${name}");
    return;
}
func bump() {                    // 省略返回注解 == unit
    count = count + 1;
}
```

- 返回类型可用 `:` 或 `->` 引出。
- **省略返回注解即 `unit`**。当前实现**不**从函数体推断返回类型：即使函数体只有
  一个 `return expr;`，签名也是 `() -> ()`，返回值被丢弃且不报错。需要返回值时
  必须显式标注（这一点与设计文档 §2.2 的"单 return 可推断"不一致，见附录 A）。
- 已标注的返回类型是**强制的**：返回表达式类型不符报 `type mismatch in return value`。
- 函数可以前向引用与递归：发射前先收集所有顶层签名。
- 首版**不支持同名重载**。

## 9.2 参数与返回值

- 形参是 **borrowed**：被调函数不得释放形参。
- 引用类型的返回值是 **owned**：向调用者交付 +1（借用值在 return 面物化 retain），
  由调用者负责结算。详见 §23 的所有权协议。

## 9.3 类型化可变参数

参数表末尾可放一个 `name...: Array<T>`；调用处逐个传参，由**调用方**打包成数组：

```sloth
func sum(xs...: Array<int>): int {
    var t = 0;
    for x in xs { t = t + x; }
    return t;
}
sum(1, 2, 3);   // 编译为 sum([1, 2, 3])
sum();          // 空包 -> 空数组
```

## 9.4 示例

```sloth
{{#include examples/functions.sl}}
```

```mlir
{{#include examples/functions.mlir}}
```

- 每个顶层函数发射为 `func.func @sloth_<模块>__<名字>`，参数与返回都是 `i64` 词。
- 可变参数 `sum(1,2,3,4)` 在调用点发射 `__sloth_arr_new_k(n, elref)` + 多次
  `__sloth_arr_set`，再把数组句柄作为唯一实参传入。
- `unit` 函数在 Sloth 层签名是 `() -> ()`，发射到 MLIR 时不带结果箭头（即无返回值）。
