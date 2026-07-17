// 本练习探索 `Cow`（写时克隆）智能指针。它可以封装借用数据并提供不可变访问，
// 在需要修改或取得所有权时再延迟克隆数据。该类型通过 `Borrow` trait 处理一般的借用数据。

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // 如果尚未拥有所有权，就克隆到一个向量中。
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // 由于需要修改 `input`，因此会发生克隆。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // 由于不需要修改 `input`，因此不会发生克隆。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO：将 `todo!()` 替换为 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_no_mutation() {
        // 我们也可以不带 `&` 传入 `vec`，让 `Cow` 直接拥有它。
        // 在这种情况下不会发生修改（所有数字都已经是绝对值），因此也不会克隆。
        // 但结果仍然拥有所有权，因为它从未被借用或修改。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO：将 `todo!()` 替换为 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_mutation() {
        // 如果确实发生了修改（并非所有数字都是绝对值），情况也一样。
        // 此时，`abs_all` 函数中的 `to_mut()` 调用会返回指向原有数据的引用。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO：将 `todo!()` 替换为 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }
}
