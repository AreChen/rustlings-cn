trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: 为字符串向量实现 trait `AppendBar`。
// `append_bar` 应将字符串 "Bar" 推入向量。

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
