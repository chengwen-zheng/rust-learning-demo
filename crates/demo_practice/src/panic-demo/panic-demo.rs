#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // 填空
        fn drink(beverage: &str) {
            if beverage == "lemonade" {
                println!("Success!");
                // 实现下面的代码
                panic!("crash down.");
            }

            println!("Exercise Failed if printing out this line!");
        }

        drink("lemonade");

        println!("Exercise Failed if printing out this line!");
    }
    #[allow(unused_variables)]
    #[test]
    fn test1() {
        // 修复所有的 panic，让代码工作
        fn divide(x: u8, y: u8) {
            if y == 0 {
                println!("Cannot divide by zero!");
            } else {
                println!("{}", x / y)
            }
        }

        // fn production_rate_per_hour(speed: u8) -> f64 {
        //     let cph: u16 = 221; // Use u16 instead of u8 to prevent overflow
        //     match speed {
        //         1..=4 => (speed as u16 * cph) as f64,
        //         5..=8 => (speed as u16 * cph) as f64 * 0.9,
        //         9..=10 => (speed as u16 * cph) as f64 * 0.77,
        //         _ => 0 as f64,
        //     }
        // }

        fn production_rate_per_hour(speed: u8) -> f64 {
            let cph: u8 = 221;
            match speed {
                1..=4 => (speed.checked_mul(cph).unwrap_or(0) as f64),
                5..=8 => (speed.checked_mul(cph).unwrap_or(0) as f64 * 0.9),
                9..=10 => (speed.checked_mul(cph).unwrap_or(0) as f64 * 0.77),
                _ => 0 as f64,
            }
        }

        pub fn working_items_per_minute(speed: u8) -> u32 {
            production_rate_per_hour(speed).round() as u32 / 60
        }

        assert_eq!("abc".as_bytes(), [97, 98, 99]);

        let v = vec![1, 2, 3];
        // let ele = v[3];
        let ele = v.get(3);

        if let Some(ele) = ele {
            println!("Element: {}", ele);
        } else {
            println!("Index out of bounds!");
        }

        // 大部分时候编译器是可以帮我们提前发现溢出错误，并阻止编译通过。但是也有一些时候，这种溢出问题直到运行期才会出现
        let v = production_rate_per_hour(2);

        divide(15, 0);

        println!("Success!")
    }
}
