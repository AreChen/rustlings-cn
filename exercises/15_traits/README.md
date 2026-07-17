# Trait（特征）

trait 是一组方法的集合。

数据类型可以实现 trait。为此，需要为数据类型定义组成该 trait 的方法。
例如，`String` 数据类型实现了 `From<&str>` trait，因此可以写出 `String::from("hello")`。

从这个角度看，trait 与 Java 接口和 C++ 抽象类有些相似。

Rust 中还有一些常见的 trait：

- `Clone`（`clone` 方法）
- `Display`（允许通过 `{}` 进行格式化显示）
- `Debug`（允许通过 `{:?}` 进行格式化显示）

由于 trait 表示数据类型之间的共享行为，因此编写泛型时非常有用。

## 延伸阅读

- [Trait](https://doc.rust-lang.org/book/ch10-02-traits.html)
