// 基础学习
mod basic_learning {
    // 2.猜数字游戏
    pub mod guess_number;
}

fn main() {
    basic_learning::guess_number::run_game();
}