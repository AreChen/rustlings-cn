// TODO: 不将宏定义移出此模块，修复编译器错误。
mod macros {
    macro_rules! my_macro {
        () => {
            println!("看看我的宏！");
        };
    }
}

fn main() {
    my_macro!();
}
