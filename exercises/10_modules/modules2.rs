// 你可以使用 `use` 和 `as` 关键字将模块路径引入作用域，并为它们提供新名称。

mod delicious_snacks {
    // TODO: 修复下面两条 `use` 语句后，将它们添加到这里。
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "最喜欢的零食：{} 和 {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
