fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: 从数组 `a` 中取出一个名为 `nice_slice` 的切片，使测试通过。
        // let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice);
    }
}
