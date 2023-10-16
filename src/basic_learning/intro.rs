pub mod match_test {
    pub fn base() {
        // 的匹配必须要穷举出所有可能
        // 每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
        enum Direction {
            East,
            West,
            North,
            South,
        }

        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            // 类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
            Direction::North | Direction::South => {
                println!("South or North");
            }
            // 类似于 switch 中的 default
            _ => println!("West"),
        };

        // match 本身也是一个表达式，因此可以用它来赋值
        let a = match Direction::East {
            Direction::East => "East",
            // 类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
            Direction::North | Direction::South => "South or North",
            // 类似于 switch 中的 default
            _ => "West",
        };
        dbg!(a);
    }

    // 模式绑定
    pub fn pattern_bind() {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
            // ...
        }
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState), // 25美分硬币
        }
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    dbg!(&state);
                    25
                }
            }
        }
        let a = value_in_cents(Coin::Dime);
        dbg!(a);
        let b = value_in_cents(Coin::Quarter(UsState::Alaska));
        dbg!(b);
    }

    pub fn wildcard() {
        let some_u8_value = 88u8;
        let a = match some_u8_value {
            1 => "one",
            3 => "three",
            5 => "five",
            7 => "seven",
            // _ 将会匹配所有遗漏的值
            _ => "else",
        };
        dbg!(a);

        let b = match some_u8_value {
            1 => "one".to_string(),
            3 => "three".to_string(),
            5 => "five".to_string(),
            7 => "seven".to_string(),
            // 用一个变量来承载其他情况
            e => {
                let e_str = e.to_string();
                format!("其他数值: {}", e_str).to_string()
            }
        };
        dbg!(b);
    }
}

pub mod if_let {
    pub fn run() {
        // 当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。
        let n = 4;
        func(n);
        fn func(n: i32) {
            let result = if 3 == n {
                println!("three");
                1
            } else { 10 };
            dbg!(result);
        }
    }
}

pub mod matches{
    pub fn run(){
        enum MyEnum {
            Foo,
            Bar
        }


        let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
        v.iter().filter(|x| matches!(x, MyEnum::Foo));

    }
}