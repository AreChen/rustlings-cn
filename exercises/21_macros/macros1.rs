macro_rules! my_macro {
    () => {
        println!("看看我的宏！");
    };
}

fn main() {
    // TODO: 修复宏调用。
    my_macro();
}
