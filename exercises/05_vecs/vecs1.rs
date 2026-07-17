fn elems_to_vec(a: i32, b: i32, c: i32) -> Vec<i32> {
    // TODO: 返回一个包含元素 a、b、c 的向量（顺序保持不变）。
    // 使用 "vec!" 宏。
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elems_to_vec() {
        let (a, b, c) = (2, 7, 12);
        let v = elems_to_vec(a, b, c);
        assert_eq!(v[0], a);
        assert_eq!(v[1], b);
        assert_eq!(v[2], c);
    }
}
