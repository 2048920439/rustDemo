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

pub mod matches {
    pub fn run() {
        enum MyEnum {
            Foo,
            Bar,
        }

        let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
        let _ = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    }
}

pub mod option {
    pub fn run() {
        /*
           一个变量要么有值：Some(T), 要么为空：None
           enum Option<T> {
               Some(T),
               None,
           }
       */

        fn plus_one(x: Option<i32>) -> i32 {
            match x {
                None => 0,
                Some(i) => i,
            }
        }

        let a = plus_one(Some(1));
        let b = plus_one(Some(5));
        let c = plus_one(None);

        dbg!(a);
        dbg!(b);
        dbg!(c);
    }
}

// 模式列表
pub mod all_patterns {
    // 匹配字面值
    pub fn a() {
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }


    // 匹配命名变量
    pub fn b() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);
    }


    // 单分支多模式
    pub fn c() {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }


    // 通过序列 ..= 匹配值的范围
    // 序列只允许用于数字或字符类型 它们可以连续，同时编译器在编译期可以检查该序列是否为空
    pub fn d() {
        let x = 2;
        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }

        let x = 'c';
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }


    // 解构并分解值 - 结构体
    pub fn e() {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        // 同名可简写
        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);
    }

    // 解构并分解值 - 枚举
    pub fn f() {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        func(Message::Move { y: 20, x: 10 });
        func(Message::ChangeColor(0, 160, 255));

        fn func(msg: Message) {
            match msg {
                Message::Move { x, y } => {
                    println!(
                        "Move in the x direction {} and in the y direction {}",
                        x, y
                    );
                },
                Message::ChangeColor(r, g, b) => {
                    println!(
                        "Change the color to red {}, green {}, and blue {}",
                        r, g, b
                    )
                },
                _ => { println!("__") }
            }
        }
    }

    // 解构并分解值 - 嵌套的结构体和枚举
    pub fn g() {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r, g, b
                )
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!(
                    "Change the color to hue {}, saturation {}, and value {}",
                    h, s, v
                )
            }
            _ => ()
        }
    }

    // 解构并分解值 - 结构体和元组
    pub fn h() {
        struct Point {
            x: i32,
            y: i32,
        }

        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

        dbg!(feet,inches);
        dbg!(x,y);
    }

    // 解构并分解值 - 数组
    pub fn i() {
        // 定长数组
        let arr: [u16; 2] = [114, 514];
        let [x, y] = arr;
        assert_eq!(x, 114);
        assert_eq!(y, 514);


        // 不定长数组
        let arr: &[u16] = &[114, 514];

        if let [x, ..] = arr {
            assert_eq!(x, &114);
        }

        if let &[.., y] = arr {
            assert_eq!(y, 514);
        }

        let arr: &[u16] = &[];

        assert!(matches!(arr, [..]));
        assert!(!matches!(arr, [_x, ..]));
    }


    // 忽略模式中的值 - 使用 _ 忽略整个值
    pub fn j() {
        // 当你需要特定类型签名但是函数实现并不需要某个参数时。
        // 此时编译器就不会警告说存在未使用的函数参数，就跟使用命名参数一样
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }
        foo(3, 4);
    }

    // 忽略模式中的值 - 使用嵌套的 _ 忽略部分值
    pub fn k() {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {:?}", setting_value);
    }

    // 忽略模式中的值 - 使用下划线开头忽略未使用的变量
    pub fn l() {
        let _x = 5; // 编译时不会提示未使用的变量
        let y = 10;
        dbg!(y);
    }

    // 忽略模式中的值 - 用 .. 忽略剩余值
    pub fn m() {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            },
        }
    }

    // 匹配守卫提供的额外条件
    // 匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件，
    // 它能为分支模式提供更进一步的匹配条件。
    pub fn n() {
        {
            let x = Some(5);
            let y = 10;

            match x {
                Some(50) => println!("Got 50"),
                Some(n) if n == y => println!("Matched, n = {}", n),
                _ => println!("Default case, x = {:?}", x), // run
            }
        }

        {
            let x = 4;
            let y = false;

            match x {
                4 | 5 | 6 if y => println!("yes"),
                _ => println!("no"), // run
            }
        }
    }


    // @绑定
    // @运算符允许为一个字段绑定另外一个变量
    // 当你既想要限定分支范围，又想要使用分支的变量时，就可以用 @ 来绑定到一个新的变量上
    pub fn o() {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            // 通过在 3..=7 之前指定 id_variable @
            // 我们捕获了任何匹配此范围的值并同时将该值绑定到变量 id_variable 上
            Message::Hello { id: id_variable @ 3..=7 } => {
                println!("Found an id in range: {}", id_variable) // run
            },
            _ => {},
        }
    }

    // @前绑定后解构
    // 使用 @ 还可以在绑定新变量的同时，对目标进行解构
    pub fn p() {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        // 绑定新变量 `p`，同时对 `Point` 进行解构 同时产生了 p,x,y三个值
        let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
        println!("x: {}, y: {}", px, py);
        println!("{:?}", p);


        let point = Point { x: 10, y: 5 };
        if let p @ Point { x: 10, y } = point {
            println!("x is 10 and y is {} in {:?}", y, p);
        }
    }

    pub fn q() {
        match 1 {
            num @ (1 | 2) => {
                println!("{}", num);
            }
            _ => {}
        }
    }

}