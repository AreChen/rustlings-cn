fn bigger(a: i32, b: i32) -> i32 {
    // TODO: 完成这个函数，使它返回较大的数字！
    // 如果两个数字相等，返回其中任意一个都可以。
    // 不要使用：
    // - 另一个函数调用
    // - 额外的变量
}

fn main() {
    // 你可以选择在这里进行实验。
}

// 现在暂时不用在意这里 :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
