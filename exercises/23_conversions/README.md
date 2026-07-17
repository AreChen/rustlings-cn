# 类型转换

Rust 提供了多种将某一类型的值转换为另一类型的方法。

最简单的类型转换形式是类型转换表达式，用二元运算符 `as` 表示。例如，
`println!("{}", 1 + 1.0);` 无法编译，因为 `1` 是整数，而 `1.0` 是浮点数。
但是，`println!("{}", 1 as f32 + 1.0)` 应该可以编译。练习 [`conversions1`](conversions1.rs)
会介绍这一点。

Rust 还提供了便于实现类型转换的 trait。这些 trait 位于 [`convert`](https://doc.rust-lang.org/std/convert/index.html) 模块中。
具体包括：

- [`conversions2`](conversions2.rs) 介绍 `From` 和 `Into`
- [`conversions4`](conversions4.rs) 介绍 `TryFrom` 和 `TryInto`
- [`conversions5`](conversions5.rs) 介绍 `AsRef` 和 `AsMut`

此外，`std::str` 模块提供了名为 [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) 的 trait，
可以通过字符串的 `parse` 方法帮助将字符串转换为目标类型。如果为某个类型 `Person` 正确实现了它，
那么 `let p: Person = "Mark,20".parse().unwrap()` 应该既能编译，也能运行而不 panic。

这些是在***标准库内部***将数据转换为所需类型的主要方式。

## 延伸阅读

书中没有直接介绍这些内容，但标准库提供了很好的文档。

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)
