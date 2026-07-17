struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // 不要修改这个函数。
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // 这里返回 `Result` 会更好。但我们想学习如何测试可能 panic 的函数。
            panic!("矩形的宽和高必须为正数");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: 这个测试应检查矩形的尺寸是否与传给构造函数的值相同。
        let rect = Rectangle::new(10, 20);
        assert_eq!(todo!(), 10); // 检查宽度
        assert_eq!(todo!(), 20); // 检查高度
    }

    // TODO: 这个测试应检查创建宽度为负的矩形时程序是否 panic。
    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: 这个测试应检查创建高度为负的矩形时程序是否 panic。
    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
