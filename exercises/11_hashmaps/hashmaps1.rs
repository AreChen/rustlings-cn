// 需要定义一个用哈希映射表示的水果篮。键表示水果名称，值表示篮中该种
// 水果的数量。你需要放入至少 3 种不同的水果（例如 apple、banana、mango），
// 并且所有水果的总数至少为 5。

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: 声明哈希映射。
    // let mut basket =

    // 已经为你放入了两个香蕉 :)
    basket.insert(String::from("banana"), 2);

    // TODO: 在篮中放入更多水果。

    basket
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
