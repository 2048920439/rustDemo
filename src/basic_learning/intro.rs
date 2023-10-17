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

    pub fn to_enum(){
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
    pub fn implement_method_for_specific_generic_type(){
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
        let p1 = Point{x:10u32,y:20u32};
        dbg!(&p1);
        // p1.distance_from_origin(); // 报错,没有相关的方法
        let p2 = Point{x:30.999f32,y:40.0f32};
        dbg!(&p2);
        let origin = p2.distance_from_origin();
        dbg!(&origin);

    }

    // const 泛型表达式
    pub fn display_const_generics_array(){
        fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
            println!("{:?}", arr);
        }
        let arr: [i32; 3] = [1, 2, 3];
        display_array(arr);

        let arr: [i32; 2] = [1, 2];
        display_array(arr);
    }
}