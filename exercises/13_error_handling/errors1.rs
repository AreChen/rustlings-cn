// TODO: 如果传入空字符串，此函数会拒绝生成要打印在姓名牌上的文本。
// 如果它能解释问题所在，而不只是返回 `None`，就会更好。幸运的是，Rust
// 有一个类似 `Option` 的结构，可以用来表达错误情况。请修改函数签名和函数体，
// 让它返回 `Result<String, String>`，而不是 `Option<String>`。
fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // 不允许空姓名
        None
    } else {
        Some(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
