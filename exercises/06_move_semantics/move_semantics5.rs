#![allow(clippy::ptr_arg)]

// TODO: 除了添加或删除引用（字符 `&`）之外，不修改任何内容，
// 修复编译器错误。

// 不应取得所有权
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// 应取得所有权
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}
