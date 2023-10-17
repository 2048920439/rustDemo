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
    // 6.模式匹配
    pub mod match_pattern;
    // 7.方法
    pub mod method;
    // 8.泛型和特征
    pub mod intro;
}

fn main() {
    basic_learning::intro::generic::display_const_generics_array()
}