fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    // TODO: 只能通过调整测试中各行的顺序来修复编译器错误。
    // 不要添加、修改或删除任何行。
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        let z = &mut x;
        y.push(42);
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
