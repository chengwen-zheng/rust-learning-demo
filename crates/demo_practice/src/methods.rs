struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 完成 area 方法，返回矩形 Rectangle 的面积
    // 🌟🌟 方法跟函数类似：都是使用 fn 声明，有参数和返回值。但是与函数不同的是，方法定义在结构体的上下文中(枚举、特征对象也可以定义方法)，
    // 而且方法的第一个参数一定是 self 或其变体 &self 、&mut self，self 代表了当前调用的结构体实例。
    fn area(&self) -> u32 {
        &self.width * &self.height
    }
}

// 只填空，不要删除任何代码行!
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self) {
        println!("the current state is {}", &self.color);
    }

    // 1. 实现下面的关联函数 `new`,
    // 2. 该函数返回一个 TrafficLight 实例，包含 `color` "red"
    // 3. 该函数必须使用 `Self` 作为类型，不能在签名或者函数体中使用 `TrafficLight`
    pub fn new(color: String) -> TrafficLight {
        TrafficLight {
            color: "red".to_string(),
        }
    }
}

impl TrafficLight {
    // 使用 `Self` 填空
    pub fn show_state1(&self) {
        println!("the current state is {}", self.color);
    }

    // 填空，不要使用 `Self` 或其变体
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// 为 TrafficLightColor 实现所需的方法
impl TrafficLightColor {
    fn color(&self) -> String {
        match *self {
            TrafficLightColor::Red => "red".to_string(),
            TrafficLightColor::Yellow => "yellow".to_string(),
            TrafficLightColor::Green => "green".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        assert_eq!(rect1.area(), 1500);
    }

    // 🌟🌟 self 会拿走当前结构体实例(调用对象)的所有权，而 &self 却只会借用一个不可变引用，&mut self 会借用一个可变引用
    #[test]
    fn test_light() {
        let light = TrafficLight {
            color: "red".to_owned(),
        };
        // 不要拿走 `light` 的所有权
        light.show_state();
        // 否则下面代码会报错
        println!("{:?}", light);
    }

    #[test]
    fn test_light_fn() {
        let c = TrafficLightColor::Yellow;

        assert_eq!(c.color(), "yellow");

        println!("{:?}", c);
    }
}
