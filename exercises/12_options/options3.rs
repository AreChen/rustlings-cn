#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: 向这个 match 语句添加内容，修复编译器错误。
    match optional_point {
        Some(p) => println!("坐标为 {},{}", p.x, p.y),
        _ => panic!("没有匹配项！"),
    }

    println!("{optional_point:?}"); // 不要修改这一行。
}
