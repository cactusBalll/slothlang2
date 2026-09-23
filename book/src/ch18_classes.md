# 18. 类与继承

## 18.1 声明

```sloth
class Counter {
    var n: int;              // 字段需标注类型
    var step: int = 2;       // 可给默认初值
    func __init__(a: int) {  // 构造器
        this.n = a;
    }
    func bump(d: int): int {
        this.n = this.n + d;
        return this.n;
    }
}
```

- 字段用 `var`/`let` 声明，**必须标注类型**；可带默认初值。
- `this` 指向当前实例，`super` 指向基类（`super.__init__(...)`、`super.method()`）。
- 单继承：`class Dog: Animal { ... }`。
- 一个类可 `impl` 多个 trait：`class Fish impl Speaker, Display { ... }`。

## 18.2 构造器规则

- 子类**自行声明** `__init__` 时，必须显式调用 `super.__init__(...)`，否则报
  `constructor of `Dog` must call super.__init__`（无声明 ctor 的类不受强制）。
- 实例化按**祖先链 base-first** 运行所有字段的初值（基类默认值不会丢）。

## 18.3 虚分派

- 通过基类引用调用方法时，若该方法在**全程序任意继承体系**中被 override，调用点经
  对象头**虚表**按方法名槽间接分派（`sloth_obj_vtable` → `sloth_vt_get` →
  `llvm.call`）；派生类的虚表槽由其 per-class builder 解析到最派生实现。
- 全程序无覆盖时在 irgen 层**消虚**为直接调用（MLIR 管线当前无 inliner）。
  `super.m()` 恒直接调用；构造器 `__init__` / 析构钩子 `__dispose__` 不参与虚派发。
- trait 方法 / `dyn` 复用同一张虚表，但按 `(trait, method)` 槽分派（见第 19 章）。
- `is` 做类型测试；`if x is C { ... }` 在分支内把 `x` **收窄**为 `C`，可访问子类
  字段/方法。无交集的两个类做 `is` 直接编译错误。`is` 仍基于运行时 class-id。

## 18.4 对象表示

- 实例是 rc 追踪的堆对象：`sloth_obj_new(cls_id, n_fields, cascade)`，字段按索引
  访问（`sloth_obj_field` / `sloth_obj_set_field`）。
- 类信息（仅 class-id + 显示名）经 `sloth_cls_info` / `sloth_cls_name` 登记。
  运行时**不携带对象布局信息**：每类的死亡级联由 codegen 发射为
  `(payload, aux) -> i64` 例程（`@...__cascade`），经分配时的第三个参数注册为
  header `sdtor`。虚表按类**全局缓存**，不再每对象重建。
- 字段写会检查词面类型：`c.n = "s"` 报 `type mismatch in field assignment `n``。

**析构钩子 `__dispose__`**：类可声明 `func __dispose__(): unit { ... }`。codegen 把它
并进该类的死亡级联例程：强计数归零时先调用 `__dispose__`（此时字段仍有效），再逐个
`release` 引用字段，最后释放对象。继承链上的 `__dispose__` 按普通方法解析，泛型实例
同样适用。详见 §23.2b。

## 18.5 示例

```sloth
{{#include examples/classes.sl}}
```

```mlir
{{#include examples/classes.mlir}}
```

要点：`Dog` 的构造器先调 `@sloth_main_Animal____init__` 再写自身字段；`a.who()`
（`a: Animal`，运行时是 `Dog`）经对象头虚表槽间接分派到 `@sloth_main_Dog__who`；
`a is Dog` 仍用 `sloth_obj_cls_id` 判定，通过后 `a.loud` 直接按 `Dog` 字段布局读。
数组字面量 `[Animal(1), Dog(2)]` 经 LUB 上转，元素类型是 `Animal`；
引用元素写入数组时 `sloth_arr_set` 伴随 `sloth_rc_retain`。
