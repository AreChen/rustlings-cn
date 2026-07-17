# 向量

向量是 Rust 中最常用的数据结构之一。在其他编程语言中，它们通常直接称为数组；
但 Rust 的运行层次更低一些：数组存储在栈上（也就是说不能增长或缩小，大小必须在
编译时确定），而向量存储在堆上（不受这些限制）。

向量在《Rust 程序设计语言》中属于稍后的章节，但我们认为它很有用，值得提前介绍。
另一个有用的数据结构——哈希映射——会在后面讲到。

## 延伸阅读

- [使用向量存储值列表](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
