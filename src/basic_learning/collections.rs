// 当我们想拥有一个列表，里面都是相同类型的数据时，Vector将会非常有用。
pub mod vector {
    // 创建
    pub fn create() {
        // Vec::new()
        {
            // 无法从 Vec::new() 中得到任何关于类型的信息
            // 无法推导出 a 的具体类型
            let a: Vec<i32> = Vec::new();
            dbg!(a);


            // 编译器通过 b.push(1u8)，推测出 b 中的元素类型是 u8
            let mut b = Vec::new();
            b.push(1u8);
            dbg!(b);

            /*
                如果预先知道要存储的元素个数
                可以使用 Vec::with_capacity(capacity) 创建动态数组
                这样可以避免因为插入大量新数据导致频繁的内存分配和拷贝，提升性能
             */
        }

        // vec![]
        {
            let a = vec![1, 2, 3];
            dbg!(a);
        }
    }

    // 更新
    pub fn update() {
        // 声明为 mut 后，才能进行修改
        let mut a: Vec<i32> = vec![];
        dbg!(&a);
        a.push(1);
        a.push(2);
        dbg!(&a);
        a.pop();
        dbg!(&a);
    }

    // 取值
    pub fn get() {
        let a = vec![1, 2, 3, 4, 5];

        // 使用下标获取
        let third: &i32 = &a[2];
        println!("第3个元素是 {}", third);

        // 使用get函数获取 get返回的是一个 Option<&T>
        match a.get(5) {
            Some(third) => println!("第6个元素是 {third}"),
            None => println!("去你的第6个元素，根本没有！"),
        }

        /*
            直接使用下标获取如果出现越界情况会直接退出程序
            而.get则会返回None.

            当你确保索引不会越界的时候，就用索引访问，否则用 .get
         */
    }

    // 遍历
    pub fn iteration() {
        let v = vec![1, 2, 3];
        for i in &v {
            println!("{i}");
        }

        let mut v = vec![1, 2, 3];
        for i in &mut v {
            *i += 10
        }
        dbg!(v);
    }

    // 存储不同类型的元素
    pub fn multiple_types() {
        // 使用枚举实现
        {
            #[derive(Debug)]
            enum IpAddr {
                V4(String),
                V6(String)
            }
            let v = vec![
                IpAddr::V4("127.0.0.1".to_string()),
                IpAddr::V6("::1".to_string())
            ];
            dbg!(v);
        }

        // 使用特征实现
        {
            trait IpAddr {
                fn display(&self);
            }

            struct V4(String);
            impl IpAddr for V4 {
                fn display(&self) {
                    println!("ipv4: {:?}", self.0)
                }
            }
            struct V6(String);
            impl IpAddr for V6 {
                fn display(&self) {
                    println!("ipv6: {:?}", self.0)
                }
            }

            // 这里必须手动地指定类型：Vec<Box<dyn IpAddr>>，表示数组 v 存储的是特征 IpAddr 的对象
            let v: Vec<Box<dyn IpAddr>> = vec![
                Box::new(V4("127.0.0.1".to_string())),
                Box::new(V6("::1".to_string())),
            ];

            for ip in v {
                ip.display();
            }
        }

        // 在实际使用场景中，特征对象数组要比枚举数组常见很多，
        // 主要原因在于特征对象非常灵活，而编译器对枚举的限制较多，且无法动态增加类型
    }

    // 排序
    pub fn sort(){
        /*
            在 rust 里，实现了两种排序算法:
                - 稳定的排序 sort 和 sort_by
                - 非稳定排序 sort_unstable 和 sort_unstable_by。

            在 稳定 排序算法里，对相等的元素，不会对其进行重新排序。而在 不稳定 的算法里则不保证这点
            总体而言，非稳定 排序的算法的速度会优于 稳定 排序算法，同时，稳定 排序还会额外分配原数组一半的空间。
         */
        // todo 未完
    }
}