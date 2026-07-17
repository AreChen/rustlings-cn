fn factorial(num: u64) -> u64 {
    // TODO: 完成这个函数，返回 `num` 的阶乘，定义为 `1 * 2 * 3 * … * num`。
    // 参考：https://en.wikipedia.org/wiki/Factorial
    //
    // 不要使用：
    // - 提前返回（显式使用 `return` 关键字）
    // 尽量不要使用：
    // - 命令式循环（for/while）
    // - 额外变量
    // 额外挑战：不要使用：
    // - 递归
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
