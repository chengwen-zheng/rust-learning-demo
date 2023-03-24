struct Sheep {
    naked: bool,
    name: String,
}
use std::ops::Mul;

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // `Sheep` 结构体上定义的方法可以调用 `Sheep` 所实现的特征的方法
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

trait Animal {
    // 关联函数签名；`Self` 指代实现者的类型
    // 例如我们在为 Pig 类型实现特征时，那 `new` 函数就会返回一个 `Pig` 类型的实例，这里的 `Self` 指代的就是 `Pig` 类型
    fn new(name: String) -> Self;

    // 方法签名
    fn name(&self) -> String;

    fn noise(&self) -> String;

    // 方法还能提供默认的定义实现
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Animal for Sheep {
    // `Self` 被替换成具体的实现者类型： `Sheep`
    fn new(name: String) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn noise(&self) -> String {
        if self.is_naked() {
            "baaaaah?".to_string()
        } else {
            "baaaaah!".to_string()
        }
    }

    // 默认的特征方法可以被重写
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // 这里的类型注释时必须的
        let mut dolly: Sheep = Animal::new("Dolly".to_string());
        // TODO ^ 尝试去除类型注释，看看会发生什么

        dolly.talk();
        dolly.shear();
        dolly.talk();
    }

    #[test]
    fn test2() {
        // 完成两个 `impl` 语句块
        // 不要修改 `main` 中的代码
        trait Hello {
            fn say_hi(&self) -> String {
                String::from("hi")
            }

            fn say_something(&self) -> String;
        }

        struct Student {}
        impl Hello for Student {
            fn say_hi(&self) -> String {
                String::from("hi")
            }

            fn say_something(&self) -> String {
                String::from("I'm a good student")
            }
        }
        struct Teacher {}
        impl Hello for Teacher {
            fn say_hi(&self) -> String {
                String::from("Hi, I'm your new teacher")
            }

            fn say_something(&self) -> String {
                String::from("I'm not a bad teacher")
            }
        }

        let s = Student {};
        assert_eq!(s.say_hi(), "hi");
        assert_eq!(s.say_something(), "I'm a good student");

        let t = Teacher {};
        assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
        assert_eq!(t.say_something(), "I'm not a bad teacher");

        println!("Success!")
    }

    // Derive 派生
    #[test]
    fn test3() {
        // `Centimeters`, 一个元组结构体，可以被比较大小
        #[derive(PartialEq, PartialOrd)]
        struct Centimeters(f64);

        // `Inches`, 一个元组结构体可以被打印
        #[derive(Debug)]
        struct Inches(i32);

        impl Inches {
            fn to_centimeters(&self) -> Centimeters {
                let &Inches(inches) = self;

                Centimeters(inches as f64 * 2.54)
            }
        }

        // 添加一些属性让代码工作
        // 不要修改其它代码！
        #[derive(Debug, PartialEq, PartialOrd)]
        struct Seconds(i32);

        let _one_second = Seconds(1);

        println!("One second looks like: {:?}", _one_second);
        let _this_is_true = _one_second == _one_second;
        let _this_is_true = _one_second > _one_second;

        let foot = Inches(12);

        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100.0);

        let cmp = if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

        println!("One foot is {} than one meter.", cmp);
    }

    //  在 Rust 中，许多运算符都可以被重载，事实上，运算符仅仅是特征方法调用的语法糖。例如 a + b 中的 + 是 std::ops::Add 特征的 add 方法调用，
    // 因此我们可以为自定义类型实现该特征来支持该类型的加法运算。
    #[test]
    fn test4() {
        // 实现 fn multiply 方法
        // 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
        // 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/\

        fn multiply<T: Mul<T, Output = T>>(a: T, b: T) -> T {
            a * b
        }

        assert_eq!(6, multiply(2u8, 3u8));
        assert_eq!(5.0, multiply(1.0, 5.0));

        println!("Success!")
    }
    #[test]
    fn test5() {
        // 修复错误，不要修改 `main` 中的代码!
        use std::ops;

        struct Foo;
        struct Bar;

        #[derive(Debug, PartialEq)]
        struct FooBar;

        #[derive(Debug, PartialEq)]
        struct BarFoo;

        impl ops::Add<Bar> for Foo {
            type Output = FooBar;

            fn add(self, _rhs: Bar) -> FooBar {
                FooBar
            }
        }

        impl ops::Sub<Foo> for Bar {
            type Output = BarFoo;

            fn sub(self, _rhs: Foo) -> BarFoo {
                BarFoo
            }
        }

        impl ops::Sub<Bar> for Foo {
            type Output = BarFoo;

            fn sub(self, _rhs: Bar) -> BarFoo {
                BarFoo
            }
        }

        // Implementation of `ops::Add` for `Bar`
        impl ops::Add<Foo> for Bar {
            type Output = FooBar;

            fn add(self, _rhs: Foo) -> FooBar {
                FooBar
            }
        }

        impl ops::Add<BarFoo> for FooBar {
            type Output = FooBar;

            fn add(self, _rhs: BarFoo) -> FooBar {
                self
            }
        }

        impl ops::Sub<FooBar> for BarFoo {
            type Output = Bar;

            fn sub(self, _rhs: FooBar) -> Bar {
                Bar
            }
        }

        // 不要修改下面代码
        // 你需要为 FooBar 派生一些特征来让代码工作
        assert_eq!(Foo + Bar, FooBar);
        assert_eq!(Foo - Bar, BarFoo);

        println!("Success!")
    }
}
