// Rust 编译器需要知道如何检查所提供的引用是否有效，以便在引用可能在使用前
// 离开作用域时提醒程序员。记住，引用是借用，不拥有自己的数据。如果它的所有者
// 离开作用域，会发生什么？

// TODO: 更新函数签名，修复编译器错误。
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
