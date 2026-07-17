# Option 类型

Option 类型表示一个可选值：每个 Option 要么是包含值的 Some，要么是不包含值的 None。
Option 类型在 Rust 代码中很常见，因为它有许多用途：

- 初始值
- 定义域不覆盖全部输入范围的函数的返回值（部分函数）
- 用于报告简单错误的返回值，发生错误时返回 None
- 可选的结构体字段
- 可以借用或“取出”的结构体字段
- 可选的函数参数
- 可为空指针
- 在棘手的情况下暂时替换值

## 延伸阅读

- [Option 枚举语法](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option 模块文档](https://doc.rust-lang.org/std/option/)
- [Option 枚举文档](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
