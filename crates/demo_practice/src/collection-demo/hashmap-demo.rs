#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test() {
        // 填空并修复错误
        let mut scores = HashMap::new();
        scores.insert("Sunface", 98);
        scores.insert("Daniel", 95);
        scores.insert("Ashley", 69);
        scores.insert("Katie", 58);

        // get 返回一个 Option<&V> 枚举值
        let score = scores.get("Sunface");
        assert_eq!(score, Some(&98));

        if scores.contains_key("Daniel") {
            // 索引返回一个值 V
            let score = scores["Daniel"];
            assert_eq!(score, 95);
            scores.remove("Daniel");
        }

        assert_eq!(scores.len(), 3);

        for (name, score) in scores {
            println!("The score of {} is {}", name, score)
        }
    }

    #[test]
    fn test1() {
        use std::collections::HashMap;
        let teams = [
            ("Chinese Team", 100),
            ("American Team", 10),
            ("France Team", 50),
        ];

        let mut teams_map1 = HashMap::new();
        for team in &teams {
            teams_map1.insert(team.0, team.1);
        }

        // 使用两种方法实现 team_map2
        // 提示:其中一种方法是使用 `collect` 方法
        // let teams_map2: HashMap<&str, i32> = teams_map1.clone().into_iter().collect();
        let teams_map2 = HashMap::from(teams);

        assert_eq!(teams_map1, teams_map2);

        println!("Success!")
    }
    #[test]
    fn test2() {
        // 填空
        use std::collections::HashMap;
        // 编译器可以根据后续的使用情况帮我自动推断出 HashMap 的类型，当然你也可以显式地标注类型：HashMap<&str, u8>
        let mut player_stats = HashMap::new();

        // 查询指定的 key, 若不存在时，则插入新的 kv 值
        player_stats.entry("health").or_insert(100);

        assert_eq!(player_stats["health"], 100);

        // 通过函数来返回新的值
        player_stats
            .entry("health")
            .or_insert_with(random_stat_buff);
        assert_eq!(player_stats["health"], 100);

        let health = player_stats.entry("health").or_insert(50);

        assert_eq!(health, &100);
        *health -= 50;
        assert_eq!(*health, 50);

        println!("Success!");

        fn random_stat_buff() -> u8 {
            // 为了简单，我们没有使用随机，而是返回一个固定的值
            42
        }
    }

    #[test]
    fn test3() {
        // 修复错误
        // 提示: `derive` 是实现一些常用特征的好办法
        use std::collections::HashMap;

        #[derive(Debug, Hash, PartialEq, Eq)]
        struct Viking {
            name: String,
            country: String,
        }

        impl Viking {
            fn new(name: &str, country: &str) -> Viking {
                Viking {
                    name: name.to_string(),
                    country: country.to_string(),
                }
            }
        }

        // 使用 HashMap 来存储 viking 的生命值
        let vikings = HashMap::from([
            (Viking::new("Einar", "Norway"), 25),
            (Viking::new("Olaf", "Denmark"), 24),
            (Viking::new("Harald", "Iceland"), 12),
        ]);

        // 使用 derive 的方式来打印 viking 的当前状态
        for (viking, health) in &vikings {
            println!("{:?} has {} hp", viking, health);
        }
    }
    #[test]
    fn test4() {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
        map.insert(1, 2);
        map.insert(3, 4);
        // 事实上，虽然我们使用了 100 容量来初始化，但是 map 的容量很可能会比 100 更多
        assert!(map.capacity() >= 100);

        // 对容量进行收缩，你提供的值仅仅是一个允许的最小值，实际上，Rust 会根据当前存储的数据量进行自动设置，当然，这个值会尽量靠近你提供的值，同时还可能会预留一些调整空间

        map.shrink_to(50);
        assert!(map.capacity() >= 50);

        // 让 Rust  自行调整到一个合适的值，剩余策略同上
        map.shrink_to_fit();
        assert!(map.capacity() >= 2);
        println!("Success!")
    }

    #[test]
    fn test5() {
        // 修复错误，尽可能少的去修改代码
        // 不要移除任何代码行！
        use std::collections::HashMap;
        let v1 = 10;
        let mut m1 = HashMap::new();
        m1.insert(v1, v1);
        println!("v1 is still usable after inserting to hashmap : {}", v1);

        // let v2 = "hello".to_string();
        let v2 = "hello";
        let mut m2 = HashMap::new();
        // 所有权在这里发生了转移
        m2.insert(v2, v1);

        assert_eq!(v2, "hello");

        println!("Success!")
    }
}
