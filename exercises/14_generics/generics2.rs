// 这个强大的包装器可以存储正整数值。
// TODO: 使用泛型重写它，使它支持包装任意类型。
struct Wrapper {
    value: u32,
}

// TODO: 调整结构体的实现，使其对被包装的值使用泛型。
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
