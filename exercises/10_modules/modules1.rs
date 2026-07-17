// TODO: 修复调用私有函数时产生的编译器错误。
mod sausage_factory {
    // 不要让这个模块之外的任何人看到它！
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("香肠！");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
