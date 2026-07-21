// 本测验测试：
// - Generics
// - Traits
//
// 一所虚构的魔法学校有一个用 Rust 编写的新成绩单生成系统！目前系统只支持创建
// 用数字表示成绩的成绩单（例如 1.0 -> 5.5）。但是学校也会颁发字母成绩（A+ -> F-），
// 因此需要能够打印这两种成绩单！
//
// 修改结构体 `ReportCard` 和 impl 块中的代码，使系统除了数字成绩单之外也支持字母成绩单。

// TODO: 按照上面的描述调整结构体。
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO: 按照上面的描述调整 impl 块。
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            self.student_name, self.student_age, self.grade,
        )
    }
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
