// 字符串
pub mod str {
    // 转换
    pub fn conversion() {
        // 将 &str 类型转为 String 类型
        {
            "hello,world".to_string();
        }
        // 将 String 类型转为 &str 类型
        {
            let s = String::from("hello,world!");
            say_hello(&s);
            say_hello(&s[..]);
            say_hello(s.as_str());
            fn say_hello(s: &str) {
                println!("{}", s);
            }
        }
    }

    // 追加 str
    pub fn push() {
        // 字符串追加操作要修改原来的字符串，则该字符串必须是可变的，即字符串变量必须由 mut 关键字修饰。
        let mut s = String::from("Hello ");
        println!("{}", s);
        s.push_str("Rust");
        println!("{}", s);
        s.push('!');
        println!("{}", s);
    }

    // 插入 str
    pub fn insert() {
        let mut s = String::from("HelloRust!");
        dbg!(&s);
        s.insert(5, ' ');
        dbg!(&s);
        s.insert_str(6, "一下插入很多内容");
        dbg!(&s);
    }

    // 替换 str
    pub fn replace() {
        // 该方法是返回一个新的字符串，而不是操作原来的字符串。
        let initial = String::from("Hello rust! rust! rust! rust!");
        dbg!(&initial);


        // 第一个参数是要被替换的字符串，第二个参数是新的字符串
        // 适用于 String 和 &str
        let s1 = initial.replace("rust", "RUST");
        dbg!(&s1);


        // 第三个参数则表示替换的个数
        // 适用于 String 和 &str
        let s2 = initial.replacen("rust", "RUST", 2);
        dbg!(&s2);


        // 仅适用于 String
        // 直接操作原来的字符串  需要使用 mut 关键字修饰
        let mut s3 = String::from("I like rust!");
        s3.replace_range(7..8, "R");
        dbg!(&s3);
    }

    // 删除 str
    pub fn delete() {
        // pop()，remove()，truncate()，clear()
        // 仅适用于 String 类型
        // 都是直接操作原字符串 需要mut修饰


        // pop
        // 返回值是一个 Option 类型，如果字符串为空，则返回 None。
        fn test_pop() {
            let mut string_pop = String::from("rust pop 中文!");
            dbg!(&string_pop);
            let p1 = string_pop.pop();
            dbg!(&p1);
            let p2 = string_pop.pop();
            dbg!(&p2);
            dbg!(&string_pop);
        }

        // remove
        // 一个参数，表示该字符起始索引位置
        // 返回值是删除位置的字符串
        fn test_remove() {
            let mut string_remove = String::from("测试remove方法");
            println!(
                "占 {} 个字节",
                std::mem::size_of_val(string_remove.as_str())
            );
            dbg!(&string_remove);
            // 删除第一个汉字
            let s1 = string_remove.remove(0);
            dbg!(&s1);

            // let r2 = string_remove.remove(1); // 报错  非法字符边界
            // 直接删除第二个汉字
            let r3 = string_remove.remove(3);
            dbg!(&r3);

            dbg!(&string_remove);
        }


        // truncate
        // 无返回值
        fn test_truncate() {
            let mut string_truncate = String::from("测试truncate");
            dbg!(&string_truncate);
            string_truncate.truncate(3);
            dbg!(&string_truncate);
        }

        // clear
        fn test_clear() {
            let mut string_clear = String::from("string clear");
            dbg!(&string_clear);
            string_clear.clear();
            dbg!(&string_clear);
        }
        test_clear()
    }

    // 连接
    pub fn concatenate() {
        // 使用 + 和 += 连接
        {
            let string_append = String::from("hello ");
            let string_rust = String::from("rust");
            // &string_rust会自动解引用为&str
            let result = string_append + &string_rust;
            let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
            result += "!!!";
            assert_eq!(result, "hello rust!!!!");


            // add会转移所有权
            let s1 = String::from("hello,");
            let s2 = String::from("world!");
            // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
            let s3 = s1 + &s2;
            assert_eq!(s3, "hello,world!");
            // 此时再使用s1则会报错
            // println!("{}",s1);
            println!("{}", s2);
        }

        // 使用 format! 连接字符串
        {
            let s1 = "hello";
            let s2 = String::from("rust");
            let s = format!("{} {}!", s1, s2);
            assert_eq!(s, "hello rust!")
        }
    }

    // 操作UTF-8字符
    pub fn operation() {
        // 使用 chars 遍历字符串
        for c in "字符串".chars() {
            println!("{}", c); // 逐个输出字符
        }

        // 使用 bytes 遍历底层字节数组
        for b in "字符串".bytes() {
            println!("{}", b);
        }
    }
}

// 结构体
pub mod struct_test {
    // 基础部分
    pub fn base() {
        // 定义一个结构
        #[derive(Debug)]
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }

        // 创建一个实例
        // 初始化实例时，每个字段都需要进行初始化
        // 初始化时的字段顺序不需要和结构体定义时的顺序一致
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("username"),
            active: true,
            sign_in_count: 1,
        };
        // 通过.读写字段
        let username = &mut user1.username;
        username.push_str("_new");
        dbg!(&user1.username);
        user1.username.push_str("_and_new");
        dbg!(&user1.username);

        // 类似js,当函数参数和结构体字段同名时，可以直接使用缩略的方式进行初始化
        fn create_user(email: String, username: String) -> User {
            User {
                email,
                username,
                active: true,
                sign_in_count: 1,
            }
        }

        let user2 = create_user("20482048@email.cc".to_string(), "init name".to_string());
        dbg!(&user2.username);

        // 类似js的 结构体更新语法
        let user2 = User {
            username: "new_name".to_string(),
            ..user2
        };
        dbg!(&user2.username);
    }

    // 元组结构体
    pub fn tuple() {
        // 结构体必须要有名称，但是结构体的字段可以没有名称
        // 这种结构体长得很像元组，因此被称为元组结构体

        #[derive(Debug)]
        struct Rgb(u8, u8, u8);

        let color_red = Rgb(255, 0, 0);
        dbg!(color_red);
    }

    // 单元结构体
    pub fn unit_like() {
        // 没有任何字段和属性
        // 如果你定义一个类型，但是不关心该类型的内容, 只关心它的行为时
        #[derive(Debug)]
        struct AlwaysEqual;

        trait SomeTrait {
            fn some_method(&self);
        }
        // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
        impl SomeTrait for AlwaysEqual {
            fn some_method(&self) {
                dbg!(&self);
            }
        }

        let subject = AlwaysEqual;
        dbg!(&subject);
        subject.some_method();
    }
}

// 枚举
pub mod enum_test {
    pub fn run() {
        // 枚举类型是一个类型，它会包含所有可能的枚举成员,
        #[derive(Debug)]
        enum PokerSuit {
            Clubs,
            Spades,
            Diamonds,
            Hearts,
        }

        #[derive(Debug)]
        struct PokerCard {
            suit: PokerSuit,
            value: u8,
        }

        // 枚举值是该类型中的具体某个成员的实例
        let heart = PokerSuit::Hearts;
        let diamond = PokerSuit::Diamonds;
        dbg!(heart);
        dbg!(diamond);


        let c1 = PokerCard { value: 1, suit: PokerSuit::Clubs };
        dbg!(c1);


        // 直接将数据信息关联到枚举成员上
        // 任何类型的数据都可以放入枚举成员中  例如字符串、数值、结构体甚至另一个枚举。
        #[derive(Debug)]
        enum PokerCardLaconic {
            Clubs(char),
            Spades(char),
            Diamonds(char),
            Hearts(char),
            // 同一个枚举类型下的不同成员还能持有不同的数据类型
            Test(String)
        }
        let c2 = PokerCardLaconic::Clubs('7');
        dbg!(c2);

    }
}

// 数组
pub mod arr {
    // 速度很快长度固定的 array
    // 可动态增长的但是有性能损耗的 Vector

    /*
        我们这里说的数组是 Rust 的基本类型，是固定长度的，
        这点与其他编程语言不同，其它编程语言的数组往往是可变长度的
        与 Rust 中的动态数组 Vector 类似
     */

    pub fn run() {
        // array 是存储在栈上  - 元素类型大小固定，且长度也是固定
        // 数组的元素类型要统一，长度要固定
        let a = [1, 2, 3, 4, 5];
        dbg!(a);
        // [类型; 长度]
        let b = [3; 5];
        dbg!(b);

        // 通过下标访问元素
        let c = a.clone();
        let first = c[0]; // 获取a数组第一个元素
        dbg!(first);
        let second = c[1]; // 获取第二个元素
        dbg!(second);


        // 数组访问越界，访问了数组中不存在的元素
        // 导致 Rust 运行时错误。程序因此退出并显示错误消息


        // 数组切片
        let a: [i32; 5] = [1, 2, 3, 4, 5];

        let slice: &[i32] = &a[1..3];

        dbg!(slice);
        // 切片的长度可以与数组不同，并不是固定的，而是取决于你使用时指定的起始和结束位置
        // 创建切片的代价非常小，因为切片只是针对底层数组的一个引用
        // 切片类型[T]拥有不固定的大小，
        //   - 而切片引用类型&[T]则具有固定的大小，
        //   - 因为 Rust 很多时候都需要固定大小数据类型，
        //   - 因此&[T]更有用,&str字符串切片也同理
    }
}