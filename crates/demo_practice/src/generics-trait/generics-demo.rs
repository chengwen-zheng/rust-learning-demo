// 填空
struct A; // 具体的类型 `A`.
struct S(A); // 具体的类型 `S`.
struct SGen<T>(T); // 泛型 `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {}

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, Y> {
    x: T,
    y: Y,
}

// 为 Val 增加泛型参数，不要修改 `main` 中的代码
struct Val<T> {
    val: T,
}

impl<M> Val<M> {
    fn value(self) -> M {
        self.val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generics1() {
        // 使用非泛型函数
        reg_fn(S(A)); // 具体的类型
        gen_spec_t(SGen(A)); // 隐式地指定类型参数  `A`.
        gen_spec_i32(SGen(2)); // 隐式地指定类型参数`i32`.

        // 显式地指定类型参数 `char`
        generic::<char>(SGen('3'));

        // 隐式地指定类型参数 `char`.
        generic(SGen(4));
    }

    #[test]
    fn test_generics2() {
        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));
    }

    #[test]
    fn test_generics3() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
    }

    #[test]
    fn test_generics4() {
        // 不要修改这行代码！
        let p = Point2 {
            x: 5,
            y: "hello".to_string(),
        };
    }
    #[test]
    fn test_generics5() {
        let x = Val { val: 3.0 };
        let y = Val {
            val: "hello".to_string(),
        };
        println!("{}, {}", x.value(), y.value());
    }

    #[test]
    fn test_generics6() {
        struct Point1<T, U> {
            x: T,
            y: U,
        }
        impl<T, U> Point1<T, U> {
            // 实现 mixup，不要修改其它代码！
            fn mixup<V, W>(self, b: Point1<V, W>) -> Point1<T, W> {
                return Point1 { x: self.x, y: b.y };
            }
        }

        let p1 = Point1 { x: 5, y: 10 };
        let p2 = Point1 {
            x: "Hello",
            y: '中',
        };

        let p3 = p1.mixup(p2);

        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '中');
    }
}
