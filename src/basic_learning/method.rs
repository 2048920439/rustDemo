// Rust 的方法往往跟结构体、枚举、特征(Trait)一起使用



// 定义方法
pub mod definition {
    // self、&self 和 &mut self
    /*
        self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
        &self 表示该方法对 Rectangle 的不可变借用
        &mut self 表示可变借用
     */
    pub fn a() {
        #[derive(Debug)]
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }
        impl Circle {
            // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
            // 这种方法往往用于初始化当前结构体的实例
            fn new(x: f64, y: f64, radius: f64) -> Circle {
                Circle { x, y, radius }
            }

            // Circle的方法，&self表示借用当前的Circle结构体
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }
    }

    // 在 Rust 中，允许方法名跟结构体的字段名相同
    pub fn b() {
        // 一般来说，方法跟字段同名，往往适用于实现 getter 访问器
        #[derive(Debug)]
        pub struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            pub fn new(width: u32, height: u32) -> Self {
                Rectangle { width, height }
            }
            pub fn width(&self) -> String {
                self.width.to_string() + "px"
            }
        }

        let rect = Rectangle::new(30, 50);

        dbg!(rect.width());
        dbg!(rect.width);

        // 用这种方式，我们可以把 Rectangle 的字段设置为私有属性，
        // 只需把它的 new 和 width 方法设置为公开可见
    }

    // 带有多个参数的方法
    pub fn c() {
        #[derive(Debug)]
        pub struct Rectangle {
            width: u32,
            height: u32,
        }
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    // 为枚举实现方法
    pub fn d() {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(u8, u8, u8),
        }

        impl Message {
            fn call(&self) {
                match &self {
                    Message::Quit => { println!("Quit") },
                    Message::Move { x, y } => { println!("Move, x:{} y:{}", x, y) },
                    Message::Write(str) => { println!("Write, str:{}", str) },
                    Message::ChangeColor(r, g, b) => { println!("rgb({},{},{})", r, g, b) }
                }
            }
        }

        let m = Message::ChangeColor(255,255,0);
        m.call();
    }
}