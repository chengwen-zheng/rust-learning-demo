struct Container(i32, i32);

// 使用关联类型实现重新实现以下特征
// trait Contains {
//    type A;
//    type B;

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 {
        self.0
    }

    // Grab the last number.
    fn last(&self) -> i32 {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::*;

    #[test]
    fn test() {
        fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
            container.last() - container.first()
        }

        let number_1 = 3;
        let number_2 = 10;

        let container = Container(number_1, number_2);

        println!(
            "Does container contain {} and {}: {}",
            &number_1,
            &number_2,
            container.contains(&number_1, &number_2)
        );
        println!("First number: {}", container.first());
        println!("Last number: {}", container.last());

        println!("The difference is: {}", difference(&container));
    }

    #[test]
    fn test1() {
        struct Container(i32, i32);

        trait ContainsPro {
            type A;
            type B;
            fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
            fn first(&self) -> i32;
            fn last(&self) -> i32;
        }
        impl ContainsPro for Container {
            type A = i32;
            type B = i32;
            fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
                (&self.0 == number_1) && (&self.1 == number_2)
            }

            fn first(&self) -> i32 {
                self.0
            }
            fn last(&self) -> i32 {
                self.1
            }
        }

        fn difference<C: ContainsPro>(container: &C) -> i32 {
            container.last() - container.first()
        }

        let number_1 = 3;
        let number_2 = 10;

        let container = Container(number_1, number_2);

        println!(
            "Does container contain {} and {}: {}",
            &number_1,
            &number_2,
            container.contains(&number_1, &number_2)
        );
        println!("First number: {}", container.first());
        println!("Last number: {}", container.last());

        println!("The difference is: {}", difference(&container));
    }

    #[test]
    fn test2() {
        trait UsernameWidget {
            fn get(&self) -> String;
        }

        trait AgeWidget {
            fn get(&self) -> u8;
        }

        struct Form {
            username: String,
            age: u8,
        }

        impl UsernameWidget for Form {
            fn get(&self) -> String {
                self.username.clone()
            }
        }

        impl AgeWidget for Form {
            fn get(&self) -> u8 {
                self.age
            }
        }

        let form = Form {
            username: "rustacean".to_owned(),
            age: 28,
        };

        // 如果你反注释下面一行代码，将看到一个错误: Fully Qualified Syntax
        // 毕竟，这里有好几个同名的 `get` 方法 Fully Qualified Syntax（完全限定语法）
        // println!("{}", form.get());

        let username = UsernameWidget::get(&form);
        assert_eq!("rustacean".to_owned(), username);
        let age = AgeWidget::get(&form); // 你还可以使用以下语法 `<Form as AgeWidget>::get`
        assert_eq!(28, age);

        println!("Success!")
    }

    #[test]
    fn test3() {
        trait Pilot {
            fn fly(&self) -> String;
        }

        trait Wizard {
            fn fly(&self) -> String;
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) -> String {
                String::from("This is your captain speaking.")
            }
        }

        impl Wizard for Human {
            fn fly(&self) -> String {
                String::from("Up!")
            }
        }

        impl Human {
            fn fly(&self) -> String {
                String::from("*waving arms furiously*")
            }
        }

        let person = Human;

        assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
        assert_eq!(<Human as Wizard>::fly(&person), "Up!");

        assert_eq!(Human::fly(&person), "*waving arms furiously*");

        println!("Success!")
    }
    #[test]
    fn test4() {
        trait Person {
            fn name(&self) -> String;
        }

        // Person 是 Student 的 supertrait .
        // 实现 Student 需要同时实现 Person.
        trait Student: Person {
            fn university(&self) -> String;
        }

        trait Programmer {
            fn fav_language(&self) -> String;
        }

        // CompSciStudent (computer science student) 是 Programmer
        // 和 Student 的 subtrait. 实现 CompSciStudent 需要先实现这两个 supertraits.
        trait CompSciStudent: Programmer + Student {
            fn git_username(&self) -> String;
        }

        fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
            format!(
                "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
                student.name(),
                student.university(),
                student.fav_language(),
                student.git_username()
            )
        }

        struct CSStudent {
            name: String,
            university: String,
            fav_language: String,
            git_username: String,
        }
        impl Person for CSStudent {
            fn name(&self) -> String {
                self.name.clone()
            }
        }
        impl Student for CSStudent {
            fn university(&self) -> String {
                self.university.clone()
            }
        }
        impl Programmer for CSStudent {
            fn fav_language(&self) -> String {
                self.fav_language.clone()
            }
        }
        impl CompSciStudent for CSStudent {
            fn git_username(&self) -> String {
                self.git_username.clone()
            }
        }

        let student = CSStudent {
            name: "Sunfei".to_string(),
            university: "XXX".to_string(),
            fav_language: "Rust".to_string(),
            git_username: "sunface".to_string(),
        };

        // 填空
        println!("{}", comp_sci_student_greeting(&student));
    }

    #[test]
    fn test5() {
        use std::fmt;

        // defined newtype `Pretty`. it's like Symbol in javascript
        struct Pretty(String);

        impl fmt::Display for Pretty {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "\"{}\"", self.0.clone() + ", world")
            }
        }

        let w = Pretty("hello".to_string());
        println!("w = {}", w);
    }
}
