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
    // 5.流程控制语句
    pub mod flow_control;
}

fn main() {
    basic_learning::flow_control::loop_test::loop_test()
}