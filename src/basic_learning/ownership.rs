// 初步了解所有权
pub fn what() {
    // Copy trait: 如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用
    {
        let n = 10;
        let double_n = double(n);
        // 整数类型实现了 Copy trait，所以将其值赋给 double_n 不会移动所有权。
        println!("{n} * 2 = {double_n}");

        fn double(n: u32) -> u32 { n * 2 }

        // 所有的标量都实现了 Copy trait
        // 当元组的所有元素都实现了Copy trait则该元组也支持Copy trait
    }

    // Drop trait: 出栈时自动清理变量
    {
        let s1 = String::from("hello");

        // 将s1的所有权转移给了calculate_length函数
        // 通过s2 接收会s1的所有权 此时s1的所有权已丢失
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
        // println!("{}", s1); // 所有权已转移,抛出错误
        fn calculate_length(s: String) -> (String, usize) {
            let len = s.len();
            // 通过返回值将s的所有权还给调用的作用域
            (s, len)
        }
    }
}

// 引用
pub fn references() {
    // 引用（reference）像一个指针，因为它是一个地址
    // 我们可以由此访问储存于该地址的属于其他变量的数据
    // 引用允许你使用值但不获取其所有权
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
        // 数签名使用 & 来表明参数 s 的类型是一个引用。
        fn calculate_length(s: &String) -> usize {
            // s.push_str(", word"); // 报错，引用默认是不可变的
            s.len()
        }
    }


    // 与使用 & 引用相反的操作是  解引用（dereferencing），它使用解引用运算符，*
    // 暂不讨论


    // 可变引用 &mut
    {
        let mut s2 = String::from("hello");
        println!("s2:{s2}");
        push_str(&mut s2);
        println!("s2:{s2}");

        fn push_str(s: &mut String) {
            s.push_str(", word"); // 报错，引用默认是不可变的
        }
    }


    // 如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用
    // 也不能在拥有不可变引用的同时拥有可变引用
    {
        let mut s3 = String::from("hello");
        let s3_1 = &mut s3;
        s3_1.push_str("[s3_1.push_str]");
        println!("s3: {}", s3);
        /*
            报错 已经存在该变量的引用
            let s3_2 = &mut s3;
            s3_1.push_str("[s3_2.push_str]");
            println!("s3_2: {}", s3_2);
        */
    }


    // 使用大括号来创建一个新的作用域，以允许拥有多个可变引用
    {
        let mut s4 = String::from("hello");
        {
            let s4_1 = &mut s4;
            s4_1.push_str("[s4_1.push_str]");
            println!("s4: {}", s4);
        }
        // 出栈时s4_1被销毁，于是可以创建第二个对s4的引用
        {
            let s4_2 = &mut s4;
            s4_2.push_str("[s4_2.push_str]");
            println!("s4: {}", s4);
        }
    }

    /*
    总结：
        - 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
        - 引用必须总是有效的。
    */
}

// slice  - 允许你引用集合中一段连续的元素序列
pub fn slice() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        return &s[..];
    }

    let str = "Hello world";
    let first = first_word(str);
    println!("{str}");
    println!("{first}");
}


// 练习
pub fn struct_example() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}

pub fn t2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);

    // 只能修改下面的代码!
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        s
    }
}

pub fn t5() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

pub fn t8() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // 仅修改下面这行代码，且不要使用 `_s`
    println!("{:?}", t.1);
}

pub fn t9() {
    let t = (String::from("hello"), String::from("world"));

    // 填空，不要修改其它代码
    let (ref s1, ref s2) = t;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}