// 这里还有一些简单的 Clippy 修复，让你了解它的用途。
// TODO: 修复所有 Clippy lint。

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // 假设你不知道 `my_option` 的值。
    // 如果它是 `Some`，我们希望打印其中的值。
    if my_option.is_none() {
        println!("{}", my_option.unwrap());
    }

    #[rustfmt::skip]
    let my_arr = &[
        -1, -2, -3
        -4, -5, -6
    ];
    println!("我的数组！在这里：{my_arr:?}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.resize(0, 5);
    println!("这个 Vec 是空的，看到了吗？{my_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // 交换这两个值！
    value_a = value_b;
    value_b = value_a;
    println!("值 a：{value_a}；值 b：{value_b}");
}
