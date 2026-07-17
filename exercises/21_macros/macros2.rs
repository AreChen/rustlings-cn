fn main() {
    my_macro!();
}

// TODO: 移动这个宏的完整定义，修复编译器错误。
macro_rules! my_macro {
    () => {
        println!("看看我的宏！");
    };
}
