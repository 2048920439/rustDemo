use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn run_game() {
    println!("Guess the number!");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("请输入一个1-100之间的数字");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("读取输入失败");

        // Rust 允许用一个新值来 隐藏 （Shadowing） guess 之前的值
        // 这个功能经常用于将一个类型的值转换为另一个类型的值。
        let guess: u8 = match guess.trim().parse() {
            Ok(num) if num >= 1 && num <= 100 => num, // 检查输入是否在1到100之间
            _ => {
                println!("请输入1-100之间的数字");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("猜小了!"),
            Ordering::Greater => println!("猜大了!"),
            Ordering::Equal => {
                println!("猜对了!");
                break;
            }
        }
    }
}
