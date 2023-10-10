#[allow(dead_code)] // 全局禁用未使用的代码警告

// 基础学习
mod basic_learning {
    // 2. 猜数字游戏
    pub mod guess_number;
    // 3. 常见编程概念
    pub mod common_concept;
}

fn main() {
    basic_learning::common_concept::control_for();
}