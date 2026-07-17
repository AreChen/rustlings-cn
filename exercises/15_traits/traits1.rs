// trait `AppendBar` 只有一个函数，它会向实现该 trait 的任意对象追加 "Bar"。
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: 为类型 `String` 实现 `AppendBar`。
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s：{s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
