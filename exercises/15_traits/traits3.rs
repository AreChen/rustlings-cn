trait Licensed {
    // TODO: 为 `licensing_info` 添加默认实现，让下面的两个结构体等实现者
    // 可以共享默认行为，而不必重复定义函数。
    // 默认许可证信息应为字符串 "Default license"。
    fn licensing_info(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // 不要编辑这一行。
impl Licensed for OtherSoftware {} // 不要编辑这一行。

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
