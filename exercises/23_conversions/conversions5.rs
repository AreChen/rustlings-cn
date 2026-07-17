// AsRef 和 AsMut 允许进行低成本的引用到引用转换。可以在
// https://doc.rust-lang.org/std/convert/trait.AsRef.html 和
// https://doc.rust-lang.org/std/convert/trait.AsMut.html 中了解更多信息。

// 获取给定参数的字节数（不是字符数）。
// （`.len()` 返回字符串的字节数。）
// TODO: 将 `AsRef` trait 作为合适的 trait 约束添加进来。
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().len()
}

// 获取给定参数的字符数（不是字节数）。
// TODO: 将 `AsRef` trait 作为合适的 trait 约束添加进来。
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// 使用 `as_mut()` 将一个数字平方。
// TODO: 添加合适的 trait 约束。
fn num_sq<T>(arg: &mut T) {
    // TODO: 实现函数体。
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
