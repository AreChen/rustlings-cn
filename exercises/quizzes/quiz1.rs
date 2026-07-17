// 本测验涵盖以下章节：
// - Variables
// - Functions
// - If
//
// Mary 正在购买苹果。苹果的价格计算方式如下：
// - 每个苹果价格为 2 个 rustbucks。
// - 但是，如果 Mary 购买超过 40 个苹果，整笔订单中每个苹果的价格都会降为
// 1 个 rustbuck！

// TODO: 编写一个函数，根据购买数量计算一笔苹果订单的价格。
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn main() {
    // 你可以选择在这里进行实验。
}

// 不要修改测试！
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
