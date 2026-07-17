// `Vec<T>` 对类型 `T` 具有泛型。在大多数情况下，编译器能够推断 `T`，
// 例如向向量推入具有具体类型的值之后。但在本练习中，编译器需要通过类型注解获得帮助。

fn main() {
    // TODO: 为向量 `Vec<T>` 标注类型，修复编译器错误。
    // 选择一种可以从 `u8` 和 `i8` 创建的整数类型作为 `T`。
    let mut numbers = Vec::new();

    // 不要修改下面的几行。
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
