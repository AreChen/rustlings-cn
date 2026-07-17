struct ColorRegularStruct {
    // TODO: 添加测试 `regular_structs` 所期望的字段。
    // 字段应使用什么类型？RGB 颜色的最小值和最大值是多少？
}

struct ColorTupleStruct(/* TODO: 添加测试 `tuple_structs` 所期望的字段 */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: 实例化一个普通结构体。
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: 实例化一个元组结构体。
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: 实例化一个单元结构体。
        // let unit_struct =
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
