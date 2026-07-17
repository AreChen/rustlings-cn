// 在编译时，Rust 需要知道一个类型占用多少空间。这对递归类型来说会有问题，
// 因为一个值可能包含另一个相同类型的值。为了解决这个问题，可以使用 `Box`——
// 一种用于在堆上存储数据的智能指针，它也允许我们包装递归类型。
//
// 本练习要实现的递归类型是“cons list”，一种函数式编程语言中常见的数据结构。
// cons list 中的每一项包含两个元素：当前项的值和下一项。最后一项是名为 `Nil` 的值。

// TODO: 在枚举定义中使用 `Box`，使代码能够编译。
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

// TODO: 创建一个空的 cons list。
fn create_empty_list() -> List {
    todo!()
}

// TODO: 创建一个非空的 cons list。
fn create_non_empty_list() -> List {
    todo!()
}

fn main() {
    println!("这是一个空的 cons list：{:?}", create_empty_list());
    println!(
        "这是一个非空的 cons list：{:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
