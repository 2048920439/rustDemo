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
    // 如果你要使用一个特征的方法，那么你需要将该特征引入当前的作用域中
    use std::convert::TryInto;
    use std::fmt::Display;

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
    fn notify1(_: &impl Summary1) {}

    // 给String实现Summary1置灰也可以传递给notify
    impl Summary1 for String {
        fn summarize(&self) -> String {
            format!("这是一个String的summarize实现,str为{}", &self)
        }
    }

    pub fn run_3() {
        let post = Post1 {
            title: "Rust语言简介".to_string(),
            author: "Sunface".to_string(),
            content: "Rust棒极了!".to_string()
        };
        notify1(&post);

        let str = String::from("string");
        notify1(&str);
    }


    // 特征约束(trait bound)
    // 只要这两个类型都实现了 Summary 特征即可
    fn notify2(_: &impl Summary1, _: &impl Summary1) {}

    // 两个参数是同一类型 且 实现了 Summary 特征即
    fn notify3<T: Summary1>(_: &T, _: &T) {}

    pub fn run_4() {
        let post = Post1 {
            title: "Rust语言简介".to_string(),
            author: "Sunface".to_string(),
            content: "Rust棒极了!".to_string()
        };
        notify1(&post);
        let str = String::from("string");
        // 只要实现了Summary1特征即可
        notify2(&post, &str);
        notify2(&post, &post);
        // 受泛型约束为同一类型
        notify3(&post, &post);
    }

    // 多重约束 - 参数需要同时实现 Summary1 + Display
    fn notify4<T: Summary1 + Display>(_: &T) {}

    pub fn run_5() {
        let str = String::from("string");
        notify4(&str);
    }

    // Where 约束
    fn func_use_where<T, U>(_: &T, _: &U) -> i32
        where T: Display + Clone, U: Clone + Summary1
    { 18i32 }

    // 等价
    fn func<T: Display + Clone, U: Clone + Summary1>(_: &T, _: &U) -> i32
    { 0i32 }

    // 使用特征约束有条件地实现方法或特征
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self { Self { x, y, } }
    }

    // 只有 T 同时实现了 Display + PartialOrd 的 Pair<T> 才可以拥有此方法
    impl<T> Pair<T> where T: Display + PartialOrd {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }


    pub trait Summary2 {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // 也可以有条件地实现特征
    // 给所有实现了Display特征的类型 实现Summary2特征
    impl<T: Display> Summary2 for T {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }


    // 函数返回中的 impl Trait
    // 可以通过 impl Trait 来说明一个函数返回的类型实现了某个特征
    pub fn run_6() {
        fn func() -> impl Summary2 {
            "文本" // &str实现了 Display -> 故也实现了 Summary2
        }

        let n = func();
        // 只知道返回了一个实现了 Summary 特征的对象
        let str = n.summarize();
        dbg!(str);
        // 并不知道他返回了一个 &str
        // println!("",n); // 报错
    }


    pub fn run_7() {
        let a: i32 = 10;
        let b: u16 = 100;

        let b_ = b.try_into()
            .unwrap();

        if a < b_ {
            println!("Ten is less than one hundred.");
        }
    }
}

pub mod test {
    use std::fmt;
    use std::fmt::{Debug, Display, Formatter};
    use std::ops::Add;

    pub fn test_1() {
        // 同时实现了PartialOrd和Copy特征
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];

            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        dbg!(largest(&[3, 5, 1]));

        dbg!(largest(&['d', 'b', 'g']));
    }

    // 给自定义类型实现 + 操作
    pub fn test_2() {
        // 为Point结构体派生Debug特征，用于格式化输出
        #[derive(Debug)]
        //限制类型T必须实现了Add特征，否则无法进行+操作。
        struct Point<T: Add<T, Output=T>> {
            x: T,
            y: T,
        }

        // 给Point实现Add
        impl<T: Add<T, Output=T>> Add for Point<T> {
            type Output = Point<T>;

            fn add(self, p: Point<T>) -> Point<T> {
                Point {
                    x: self.x + p.x,
                    y: self.y + p.y,
                }
            }
        }

        fn add<T: Add<T, Output=T>>(a: T, b: T) -> T {
            a + b
        }

        let p1 = Point { x: 1.1f32, y: 1.1f32 };
        let p2 = Point { x: 2.1f32, y: 2.1f32 };
        dbg!(p1 + p2);

        let p3 = Point { x: 1i32, y: 1i32 };
        let p4 = Point { x: 2i32, y: 2i32 };
        dbg!(p3 + p4);
    }

    // 自定义类型的打印输出
    pub fn test_3() {
        enum FileState { Open, Closed }
        struct File {
            name: String,
            data: Vec<u8>,
            state: FileState,
        }

        impl Display for FileState {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    FileState::Open => write!(f, "OPEN"),
                    FileState::Closed => write!(f, "CLOSED"),
                }
            }
        }

        impl Display for File {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Display: <{} ({})>", self.name, self.state)
            }
        }

        impl Debug for File {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "Debug: <{} ({})>", self.name, self.state)
            }
        }

        impl File {
            fn new(name: &str) -> File {
                File {
                    name: String::from(name),
                    data: Vec::new(),
                    state: FileState::Closed,
                }
            }
        }

        let f6 = File::new("f6.txt");
        println!("{:?}", f6);   // 调用Debug.fmt
        println!("{}", f6);     // 调用Display.fmt
    }
}