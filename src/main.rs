#[allow(dead_code)] // 全局禁用未使用的代码警告

// 基础学习
mod basic_learning {
    // 1. 猜数字游戏
    pub mod guess_number;
    // 2. 常见编程概念
    pub mod common_concept;
    // 3.所有权
    pub mod ownership;
    // 4.复合类型
    pub mod compound_type;
}

fn main() {
    basic_learning::compound_type::enum_test::base()
}