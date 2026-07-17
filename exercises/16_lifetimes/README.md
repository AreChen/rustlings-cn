# 生命周期

生命周期告诉编译器如何检查引用在特定情况下是否存活了足够长的时间，
从而保证引用有效。例如，生命周期会说：“确保参数 'a' 的存活时间不短于参数 'b'，
这样返回值才有效”。

生命周期只对借用（即引用）是必需的，因为复制的参数或移动后的值在其作用域内
拥有所有权，不能在作用域外被引用。生命周期让函数等调用代码可以经过检查，
从而确保参数有效。生命周期会约束调用者。

如果你想进一步了解生命周期注解，
[lifetimekata](https://tfpk.github.io/lifetimekata/) 项目提供了与 Rustlings
风格相似的练习，但专门用于学习编写生命周期注解。

## 延伸阅读

- [生命周期（Rust By Example）](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [使用生命周期验证引用](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
