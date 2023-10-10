use std::fmt::Display;

// 变量
pub fn variables() {
    // mut 声明变量为可变量
    let mut x: u8 = 1;
    println!("The value of x is: {x}");
    x = 2;
    println!("The value of x is: {x}");

    // 通过隐藏将y写为2,其中第一个y在无引用的情况下会被标记为可回收
    let y: u8 = 1;
    println!("The value of y is: {y}");
    let y: u8 = 2;
    println!("The value of y is: {y}");
}

// 常量
pub fn concept() {
    /*
    Rust 对常量的[命名约定]是在单词之间使用全大写加下划线
    编译器能够在编译时[计算一组有限的操作]
    这使我们可以选择以更容易理解和验证的方式写出此值
    在声明它的[作用域]之中，常量在整个程序生命周期中都有效
    */
    const MILLISECONDS_IN_A_DAY: u32 = 1000 * 60 * 60 * 24;
    println!("1天等于{MILLISECONDS_IN_A_DAY} 毫秒");
}

// 隐藏
pub fn shadowing() {
    let x = 5;

    let x = x + 1; // 将x设置为6

    {
        let x = x * 2;
        // 12 使用了作用域内部的x
        println!("作用域内部 {x}");
    }

    // 虽然上方代码块中将x写为了12,但是当前作用域内的x依旧为6
    println!("作用域外部: {x}");
}

// 数据类型 - 标量
pub fn data_type_scalar() {
    println!("具体内容看");
    println!("https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html")
    /*
    整数
    长度分别为 8/16/32/64/128/arch
        - 有符号i 无符号u
            - 无符号的变体可以储存从 0 到 2^n - 1 的数字
            - 有符号的变体可以储存包含从 -(2^(n - 1)) 到 (2^(n - 1)) - 1 在内的数字
            - 例:
                - u8可以表示 [0..=255]  -> 2^8 - 1 = 255
                - i8可以表示 [-128..=127 ]-> -(2^7) = -128  ,   2^7 - 1 = 127
        - arch --> isize 和 usize类型依赖运行程序的计算机架构：
            - 64 位架构上它们是 64 位的，32 位架构上它们是 32 位的。
    */

    /*
    浮点
    - f64（双精度浮点数）可以表示的数字范围大约是 ±2.23 x 10^(-308) 到 ±1.80 x 10^(308)。
    - f32（单精度浮点数）可以表示的数字范围大约是 ±1.18 x 10^(-38) 到 ±3.40 x 10^(38)。
     */

    /*
    布尔
    true false
     */

    /*
    字符
        -- 使用单引号声明 char字面量
        -- 使用双引号声明字符串字面量
     */
}

// 数据类型 - 复合类型
pub fn data_type_compound() {
    // 原生的复合类型：元组（tuple）和数组（array）。


    // 元组   是一个将多个其他类型的值组合进一个复合类型的主要方式。

    // 元组长度固定：一旦声明，其长度不会增大或缩小。
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 可以使用模式匹配（pattern matching）来解构（destructure）元组值
    {
        let (a, b, c) = tup;
        println!("解构元组:");
        println!("a:{a},  b:{b},  c:{c}");
    }
    // 也可以使用索引获取元组中的数据
    {
        let a = tup.0;
        let b = tup.1;
        let c = tup.2;
        println!("使用下标读取元组:");
        println!("a:{a},  b:{b},  c:{c}");
    }


    // 数组
    // 数组中的每个元素的类型必须相同
    // 数组长度是固定的
    {
        let a = [1, 2, 3, 4, 5];
        let b = ["1", "2", "3"];
        let a = array_to_csv(&a);
        let b = array_to_csv(&b);
        println!("定义数组:");
        println!("a: {a}");
        println!("b: {b}");
    }

    // 还可以通过在方括号中指定初始值加分号再加元素个数的方式来创建一个每个元素都为相同值的数组
    {
        let a = [3; 5];
        let b = [3, 3, 3, 3, 3];

        let a = array_to_csv(&a);
        let b = array_to_csv(&b);
        println!("声明相同值:");
        println!("a: {a}");
        println!("b: {b}");
    }

    // 读取数组
    {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let a_as_csv = array_to_csv(&a);
        println!("完整数组: {a_as_csv}");

        // 使用下标读取数据
        let first = a[0];
        println!("通过下标读取第一位: {first}");
    }
}

// 使用GPT生成的工具函数,用于简化数组打印
fn array_to_csv<T: Display>(array: &[T]) -> String {
    array.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
}

// 函数
pub fn functions() {

    // Rust 不关心函数定义所在的位置
    fn1();
    fn fn1() {
        println!("fn1 执行了")
    }

    // 参数
    fn fn2(x: u32, str: String) {
        // 在函数签名中，必须声明每个参数的类型
        println!("fn2 接受一个 u32 类型的参数, 参数为: {}", x);
        println!("fn2 接受一个 String 类型的参数, 参数为: {}", str);
    }
    fn2(888, String::from("string"));

    // 语句（Statements）是执行一些操作但不返回值的指令。
    // 表达式（Expressions）计算并产生一个值。让我们看一些例子。
    // 表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句

    // 返回值
    // 我们并不对返回值命名，但要在箭头（->）后声明它的类型
    // 函数的返回值等同于函数体最后一个表达式的值
    fn fn3(x: u32) -> u32 {
        x / 2 // 等同于 return x / 2;
    }
    println!("fn3(80)的返回值为 {}", fn3(80))
}

// 控制流 if else
pub fn control_if() {
    // if 表达式
    let a = 6;
    if a > 8 {
        println!("a > 8")
    } else if a > 5 {
        println!("a > 5")
    } else {
        println!("a <= 5")
    }

    // Rust 并不会尝试自动地将非布尔值转换为布尔值。
    // 必须总是显式地使用布尔值作为 if 的条件。
    if a != 0 { println!("a 不为 0") }

    // if 是一个表达式，我们可以在 let 语句的右侧使用它
    // 代码块的值是其最后一个表达式的值，而数字本身就是一个表达式
    let number = if a > 10 { 5 } else { 6 };
    println!("{number}") // 6
}

// loop 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止。
pub fn control_loop() {
    let mut i = 0;
    // 使用 break 关键字来告诉程序何时停止循环
    loop {
        i += 1;
        if i > 5 {
            println!("i > 5 停止循环");
            break;
        } else if i == 2 {
            println!("i = 2 跳过本次循环");
            continue;
        }
        println!("执行 i = {i}");
    }

    // 如果将返回值加入你用来停止循环的 break 表达式，它会被停止的循环返回
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 { break counter; }
    };
    println!("result的值为: {result}");


    println!("------------");
    // 循环标签
    // 如果存在嵌套循环，break 和 continue 应用于此时最内层的循环
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// 当条件为 true，执行循环
pub fn control_while() {
    let mut i = 0;
    while i < 10 {
        i += 1;
        println!("i = {i}")
    }
    println!("停止执行, i = {i}")
}

// 使用 for 循环遍历集合中的元素
pub fn control_for() {
    let arr = [1, 2, 3, 4, 5, 6];
    for item in &arr { println!("{item}") }


    // 大部分 Rustacean 也会使用 for 循环。这么做的方式是使用 Range
    for item in 10..=15 {
        println!("{item}")
    }
}
