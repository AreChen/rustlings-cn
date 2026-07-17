// 测试很重要，它能确保代码的行为符合你的预期。

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    // TODO: 导入 `is_even`。你可以使用通配符导入外部模块中的所有内容。

    #[test]
    fn you_can_assert() {
        // TODO: 使用一些值测试函数 `is_even`。
        assert!();
        assert!();
    }
}
