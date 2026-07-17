// TODO: 不修改 `main` 函数，修复其中的编译器错误。
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // 不要修改这一行。

    if is_a_color_word(word) {
        println!("这是我认识的颜色词！");
    } else {
        println!("这不是我认识的颜色词。");
    }
}
