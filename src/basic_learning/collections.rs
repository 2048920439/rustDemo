// 当我们想拥有一个列表，里面都是相同类型的数据时，Vector将会非常有用。
pub mod vector {
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

    pub fn update(){
        // 声明为 mut 后，才能进行修改
        let mut a:Vec<i32> = vec![];
        dbg!(&a);
        a.push(1);
        a.push(2);
        dbg!(&a);
        a.pop();
        dbg!(&a);
    }

    pub fn get(){
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

        // todo:
        // https://course.rs/basic/collections/vector.html#%E5%90%8C%E6%97%B6%E5%80%9F%E7%94%A8%E5%A4%9A%E4%B8%AA%E6%95%B0%E7%BB%84%E5%85%83%E7%B4%A0
    }
}