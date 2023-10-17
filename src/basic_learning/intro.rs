// 在 Rust 中泛型是零成本的抽象，意味着你在使用泛型时，完全不用担心性能上的问题。
// Rust 是在编译期为泛型对应的多个类型，生成各自的代码，因此损失了编译速度和增大了最终生成文件的大小。
pub mod generic {
    pub fn to_struct() {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        dbg!(integer,float);
    }

    pub fn to_enum() {
        #[derive(Debug)]
        enum Option<T> {
            Some(T),
            None,
        }
        #[derive(Debug)]
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
        let some = Option::Some(18u8);
        dbg!(some);
        let ok = Option::Some(18u32);
        dbg!(ok);
    }

    pub fn to_fn() {
        // 该特征的目的就是让类型实现可比较的功能 : std::ops::Add<Output = T>
        fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
            a + b
        }

        println!("add i8: {}", add(2i8, 3i8));
        println!("add i32: {}", add(20, 30));
        println!("add f64: {}", add(1.23, 1.23));
    }

    // 为具体的泛型类型实现方法
    pub fn implement_method_for_specific_generic_type() {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        let p1 = Point { x: 10u32, y: 20u32 };
        dbg!(&p1);
        // p1.distance_from_origin(); // 报错,没有相关的方法
        let p2 = Point { x: 30.999f32, y: 40.0f32 };
        dbg!(&p2);
        let origin = p2.distance_from_origin();
        dbg!(&origin);
    }

    // const 泛型表达式
    pub fn display_const_generics_array() {
        fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
            println!("{:?}", arr);
        }
        let arr: [i32; 3] = [1, 2, 3];
        display_array(arr);

        let arr: [i32; 2] = [1, 2];
        display_array(arr);
    }
}

// 特征定义了一组可以被共享的行为，只要实现了特征，你就能使用这组行为。
pub mod trait_test {
    // 定义特征
    pub trait Summary {
        // 使用 trait 关键字来声明一个特征
        // 大括号中定义了该特征的所有方法
        fn summarize(&self) -> String;
    }

    pub struct Post {
        pub title: String,
        // 标题
        pub author: String,
        // 作者
        pub content: String, // 内容
    }

    pub struct Weibo {
        pub username: String,
        pub content: String
    }

    // 为类型实现特征
    // 为 Post 类型实现 Summary 特征
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }

    // 为 Weibo 类型实现 Summary 特征
    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }

    pub fn run_1() {
        let post = Post {
            title: "Rust语言简介".to_string(),
            author: "Sunface".to_string(),
            content: "Rust棒极了!".to_string()
        };
        let weibo = Weibo {
            username: "sunface".to_string(),
            content: "好像微博没Tweet好用".to_string()
        };

        println!("{}", post.summarize());
        println!("{}", weibo.summarize());
    }

    // 孤儿规则:
    /*
        如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！
        https://course.rs/basic/trait/trait.html#%E7%89%B9%E5%BE%81%E5%AE%9A%E4%B9%89%E4%B8%8E%E5%AE%9E%E7%8E%B0%E7%9A%84%E4%BD%8D%E7%BD%AE%E5%AD%A4%E5%84%BF%E8%A7%84%E5%88%99
     */


    // 默认实现
    // 默认实现允许调用相同特征中的其他方法，哪怕这些方法没有默认实现。
    pub trait Summary1 {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct Post1 {
        pub title: String,
        // 标题
        pub author: String,
        // 作者
        pub content: String, // 内容
    }

    pub struct Weibo1 {
        pub username: String,
        pub content: String
    }

    impl Summary1 for Post1 {
        // 重写覆盖默认实现
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }

    // 使用默认实现
    impl Summary1 for Weibo1 {}

    pub fn run_2() {
        let post = Post1 {
            title: "Rust语言简介".to_string(),
            author: "Sunface".to_string(),
            content: "Rust棒极了!".to_string()
        };
        let weibo = Weibo1 {
            username: "sunface".to_string(),
            content: "好像微博没Tweet好用".to_string()
        };

        println!("{}", post.summarize());
        println!("{}", weibo.summarize());
    }


    // 使用特征作为函数参数
    pub fn notify(item: &impl Summary1) {
        println!("Breaking news! {}", item.summarize());
    }
    pub fn run_3(){
        let post = Post1 {
            title: "Rust语言简介".to_string(),
            author: "Sunface".to_string(),
            content: "Rust棒极了!".to_string()
        };
        notify(&post);

        // 给String实现Summary1置灰也可以传递给notify
        impl Summary1 for String{
            fn summarize(&self) -> String{
                format!("这是一个String的summarize实现,str为{}",&self)
            }
        }
        let str = String::from("string");
        notify(&str);
    }
}