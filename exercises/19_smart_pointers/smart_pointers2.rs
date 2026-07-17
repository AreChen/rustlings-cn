// 本练习使用 `Rc<T>` 类型表达多个所有者的概念。这是一个太阳系模型，其中有
// 一个 `Sun` 类型和多个 `Planet`。行星取得太阳的所有权，表示它们围绕太阳运行。

use std::rc::Rc;

#[derive(Debug)]
struct Sun;

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("来自 {self:?} 的问候！");
    }
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc1() {
        let sun = Rc::new(Sun);
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 1 个引用

        let mercury = Planet::Mercury(Rc::clone(&sun));
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 2 个引用
        mercury.details();

        let venus = Planet::Venus(Rc::clone(&sun));
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 3 个引用
        venus.details();

        let earth = Planet::Earth(Rc::clone(&sun));
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 4 个引用
        earth.details();

        let mars = Planet::Mars(Rc::clone(&sun));
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 5 个引用
        mars.details();

        let jupiter = Planet::Jupiter(Rc::clone(&sun));
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 6 个引用
        jupiter.details();

        // TODO
        let saturn = Planet::Saturn(Rc::new(Sun));
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 7 个引用
        saturn.details();

        // TODO
        let uranus = Planet::Uranus(Rc::new(Sun));
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 8 个引用
        uranus.details();

        // TODO
        let neptune = Planet::Neptune(Rc::new(Sun));
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 9 个引用
        neptune.details();

        assert_eq!(Rc::strong_count(&sun), 9);

        drop(neptune);
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 8 个引用

        drop(uranus);
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 7 个引用

        drop(saturn);
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 6 个引用

        drop(jupiter);
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 5 个引用

        drop(mars);
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 4 个引用

        // TODO
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 3 个引用

        // TODO
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 2 个引用

        // TODO
        println!("引用计数 = {}", Rc::strong_count(&sun)); // 1 个引用

        assert_eq!(Rc::strong_count(&sun), 1);
    }
}
