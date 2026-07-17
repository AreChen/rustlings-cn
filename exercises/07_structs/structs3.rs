// 结构体包含数据，也可以包含逻辑。在本练习中，我们定义了 `Fireworks`
// 结构体以及与它配合使用的几个函数。请将这些独立函数改为方法和关联函数，
// 以便在代码中更清晰地表达它们之间的关系。

#![deny(clippy::use_self)] // 练习使用 `Self` 类型

#[derive(Debug)]
struct Fireworks {
    rockets: usize,
}

// TODO: 将这个函数改为 `Fireworks` 的关联函数。
fn new_fireworks() -> Fireworks {
    Fireworks { rockets: 0 }
}

// TODO: 将这个函数改为 `Fireworks` 的方法。
fn add_rockets(fireworks: &mut Fireworks, rockets: usize) {
    fireworks.rockets += rockets
}

// TODO: 将这个函数改为 `Fireworks` 的方法。
fn start(fireworks: Fireworks) -> String {
    "🚀".repeat(fireworks.rockets)
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_some_fireworks() {
        let f = Fireworks::new();
        assert_eq!(f.start(), "");

        let mut f = Fireworks::new();
        f.add_rockets(3);
        assert_eq!(f.start(), "🚀🚀🚀");

        let mut f = Fireworks::new();
        f.add_rockets(7);
        // 最后一个测试不使用方法语法，以确保 `start` 函数取得烟花的所有权。
        assert_eq!(Fireworks::start(f), "🚀🚀🚀🚀🚀🚀🚀");
    }
}
