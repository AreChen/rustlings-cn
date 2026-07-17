// Clippy 工具是一组用于分析代码的 lint，可以帮助你发现常见错误并改进 Rust 代码。
//
// 对于这些练习，只要存在 Clippy 警告，代码就会编译失败。
// 查看输出中的 Clippy 建议来完成练习。

fn main() {
    // TODO: 修复这一行中的 Clippy lint。
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("半径为 {radius:.2} 的圆面积为 {area:.5}");
}
