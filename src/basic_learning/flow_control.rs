pub mod if_else {
    pub fn run() {
        let condition = true;
        // if 语句块是表达式，这里我们使用 if 表达式的返回值来给 number 进行赋值
        let number = if condition {
            5
        } else {
            6
        };

        println!("The value of number is: {}", number);
    }
}

pub mod loop_test {

    // continue 跳出本次循环
    // break    跳出整个循环

    pub fn for_test() {

        // 循环1-5
        for _ in 1..=5 {}

        let a = [String::from("1"), String::from("2"), String::from("3")];
        // 如果不使用引用的话，所有权会被转移（move）到 for 语句块中
        // 后面就无法再使用这个集合了
        for _ in &a {}
        dbg!(a);

        let mut b = [String::from("1"), String::from("2"), String::from("3")];
        // 循环可变引用
        for i in &mut b {
            i.push_str("_new")
        }
        dbg!(&b);
    }

    pub fn while_test() {
        let mut n = 0;
        while n <= 5 {
            dbg!(n);
            n = n + 1;
        }
        println!("循环结束！");
    }

    pub fn loop_test() {
        //  loop 就是一个简单的无限循环
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {}", result);

        // break 可以单独使用，也可以带一个返回值，有些类似 return
        // loop 是一个表达式，因此可以返回一个值
    }


}